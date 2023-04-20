mod http;

use crate::charts::http::Http;
use crate::comp::http::get_top_60;
use crate::db::redis_functions::RedisDb;
use crate::ram::{CCWMS_CACHE, DPRS_CACHE, ENV, OPRS_CACHE};
use log::{error, info};
use rayon::prelude::*;
use std::process::exit;
use std::sync::Mutex;
use std::thread;
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Version {
    Match,
    Pre,
    Top60,
}

pub async fn populate(version: Version) {
    let mut teams_to_track: Vec<u16> = match version {
        Version::Match => ENV.teams.clone(),
        Version::Pre => ENV.teams.clone(),
        Version::Top60 => get_top_60().await,
    };
    let Some(redis) = RedisDb::new() else {
        error!("FAILED TO START REDIS DB WHILE GETTING OPR DATA");
        return;
    };
    let db = Mutex::new(redis);
    let works: Vec<u16> = teams_to_track
        .par_iter()
        .filter_map(|team| {
            let team_data = Http::new(*team, version)?.get_data()?;
            if !check_cache(team, &team_data.ccwms, &team_data.oprs, &team_data.dprs){
                return None;
            }
            let add_cache: http::Team = team_data.clone();
            let team_borrow: u16  = team.to_owned();
            thread::spawn( move || {
                let team = team_borrow;
                let add_cache: http::Team = add_cache;
                CCWMS_CACHE.lock().expect("Dead Lokc").insert(team, add_cache.ccwms);
                DPRS_CACHE.lock().expect("Dead Lokc").insert(team, add_cache.dprs);
                OPRS_CACHE.lock().expect("Dead Lokc").insert(team, add_cache.oprs);
            });
            let mut db = db.lock().expect("Dead Lock");
            db.set_team(team, "oprs", Some(team_data.oprs));
            db.set_team(team, "ccwms", Some(team_data.ccwms));
            db.set_team(team, "dprs", Some(team_data.dprs));
            Some(team.to_owned())
        })
        .collect();
    if teams_to_track != works && version != Version::Match {
        for x in &works {
            teams_to_track.retain(|team| team != x);
        }
        error!(
            "failed to send for {} teams, the teams were {:?}",
            teams_to_track.len(),
            teams_to_track
        );
        exit(1);
    }
    info!(
        "Sent data for all {} teams {:?}",
        teams_to_track.len(),
        teams_to_track
    )
}

fn check_cache(team : &u16, old_ccwms : &f32, old_oprs : &f32, old_drps: &f32)-> bool {
    if let Some(ccwms) = CCWMS_CACHE.lock().expect("Dead Lokc").get(team)  {
        if old_ccwms == ccwms {
            return true;
        }
    }
    if let Some(old) = OPRS_CACHE.lock().expect("Dead Lokc").get(team)  {
        if old_oprs == old {
            return true;
        }
    }
    if let Some(old) = DPRS_CACHE.lock().expect("Dead Lokc").get(team)  {
        if old_drps == old {
            return true;
        }
    }
    false
}
