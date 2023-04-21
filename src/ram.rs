use crate::comp::avg::math::YearAround;
use crate::comp::event::math::EventData;
use crate::startup::tba::Tba;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex};
pub const YEAR: u16 = 2023;

#[derive(Debug)]
pub struct Env {
    pub code: String,
    pub api_key: String,
    pub redis_key: String,
    pub sentry_dsn: String,
    pub firestore_collection: String,
    pub update_where: String,
    pub teams: Vec<u16>,
}

pub static ENV: Lazy<Env> = Lazy::new(|| {
    let api_key = dotenv!("API_KEY");
    let code = "2023cur";
    let mut teams = Tba::get_teams(code, api_key).expect("Failed While Getting Teams");
    let event_name = Tba::get_event(code, api_key).expect("Failed While Getting Event Name");
    teams = teams.iter().filter_map(|x| {
        if x == &8177{
            return  None;
        }
        Some(*x)
    }).collect();
    dotenv().ok();
    Env {
        code: code.to_owned(),
        api_key: api_key.to_owned(),
        redis_key: dotenv!("REDIS").to_owned(),
        sentry_dsn: dotenv!("SENTRY_DSN").to_owned(),
        firestore_collection: event_name,
        update_where: code.to_owned(),
        teams : teams,
    }
});

// pub static EKAMAI: Lazy<Mutex<HashMap<u16, f32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static READY: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static OPRS_CACHE: Lazy<Mutex<HashMap<u16, f32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static CCWMS_CACHE: Lazy<Mutex<HashMap<u16, f32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static DPRS_CACHE: Lazy<Mutex<HashMap<u16, f32>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static CACHE_YEAR_AVG: Lazy<Mutex<HashMap<u16, YearAround>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
pub static CACHE_MATCH_AVG: Lazy<Mutex<HashMap<u16, YearAround>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CACHE_MATCH: Lazy<Mutex<HashMap<u16, Vec<EventData>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
