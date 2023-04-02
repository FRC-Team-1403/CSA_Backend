use crate::comp::avg::math::YearAround;
use crate::comp::event::math::EventData;
use crate::startup::tba::Tba;
use dotenv::dotenv;
use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Env {
    pub api_key: String,
    pub redis_key: String,
    pub sentry_dsn: String,
    pub firestore_collection: String,
    pub update_where: String,
    pub teams: Vec<u16>,
}

pub static ENV: Lazy<Env> = Lazy::new(|| {
    let api_key = dotenv!("API_KEY");
    let code = "2023njski";
    let teams = Tba::get_teams(code, api_key).expect("Failed While Getting Teams");
    let event_name = Tba::get_event(code, api_key).expect("Failed While Getting Teams");
    dotenv().ok();
    Env {
        api_key: api_key.to_owned(),
        redis_key: dotenv!("REDIS").to_owned(),
        sentry_dsn: dotenv!("SENTRY_DSN").to_owned(),
        firestore_collection: event_name,
        update_where: code.to_owned(),
        teams,
    }
});

pub fn get_pub() -> MutexGuard<'static, HashMap<u16, YearAround>> {
    loop {
        if let Ok(data) = CACHE_YEAR_AVG.try_lock() {
            return data;
        }
        error!("FAILED WHEN LOCKING CACHE_YEAR_AVG, THIS MAY BE A DEAD LOCK!!!!");
        thread::sleep(Duration::from_millis(1000));
    }
}

pub static CACHE_YEAR_AVG: Lazy<Mutex<HashMap<u16, YearAround>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CACHE_MATCH_AVG: Lazy<Mutex<HashMap<u16, YearAround>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CACHE_MATCH: Lazy<Mutex<HashMap<u16, Vec<EventData>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
