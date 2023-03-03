use crate::comp::avg::math::YearAround;
use crate::comp::event::math::EventData;
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
}

pub static ENV: Lazy<Env> = Lazy::new(|| {
    dotenv().ok();
    Env {
        api_key: dotenv!("API_KEY").to_owned(),
        redis_key: dotenv!("REDIS").to_owned(),
        sentry_dsn: dotenv!("SENTRY_DSN").to_owned(),
        firestore_collection: "South Florida Regional".to_owned(),
        update_where: "2022flwp".to_owned(),
    }
});

pub static CACHE_MATCH_AVG: Lazy<Mutex<HashMap<u16, YearAround>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static CACHE_MATCH: Lazy<Mutex<HashMap<u16, Vec<EventData>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
