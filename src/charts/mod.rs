mod http;

use crate::charts::http::Http;
use crate::comp::http::get_top_60;
use crate::db::redis_functions::RedisDb;
use rayon::prelude::*;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;
use log::error;
use crate::ram::ENV;

pub enum  Version{
    Match,
    Pre,
}
pub async fn populate(version: Version) {
    let teams_to_track: Vec<u16> = match version {
        Version::Match => {
            ENV.teams.clone()
        }
        Version::Pre => {
            get_top_60().await
        }
    };
    let Some(redis) = RedisDb::new() else{
        error!("FAILED TO START REDIS DB WHILE GETTING OPR DATA");
        return;
    };
    let db = Mutex::new(redis);
    let _: () = teams_to_track
        .par_iter()
        .filter_map(|team| {
            let team_data = Http::new(*team)?.get_data()?;
            let mut db = lock(&db);
            db.set_team(team, "oprs", Some(team_data.oprs));
            db.set_team(team, "ccwms", Some(team_data.ccwms));
            db.set_team(team, "dprs", Some(team_data.dprs));
            Some(())
        })
        .collect();
}

pub fn lock(db: &Mutex<RedisDb>) -> MutexGuard<RedisDb> {
    loop {
        if let Ok(data) = db.try_lock() {
            return data;
        }
        thread::sleep(Duration::from_millis(1000))
    }
}
