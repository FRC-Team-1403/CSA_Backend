mod http;

use crate::charts::http::Http;
use crate::comp::http::get_top_60;
use crate::db::redis_functions::RedisDb;
use crate::ram::ENV;
use log::{error, info};
use rayon::prelude::*;
use std::process::exit;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;

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
            let mut db = lock(&db);
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

pub fn lock(db: &Mutex<RedisDb>) -> MutexGuard<RedisDb> {
    loop {
        if let Ok(data) = db.try_lock() {
            return data;
        }
        thread::sleep(Duration::from_millis(1000))
    }
}
