use crate::comp::avg::math::YearAround;
use crate::comp::event::math::EventData;
use crate::startup::tba::Tba;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

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
    let code = "2023njfla";
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

pub static CACHE_MATCH_AVG: Lazy<Mutex<HashMap<u16, YearAround>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CACHE_MATCH: Lazy<Mutex<HashMap<u16, Vec<EventData>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
