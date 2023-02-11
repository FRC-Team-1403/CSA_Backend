use dotenv::dotenv;
use once_cell::sync::Lazy;

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
        firestore_collection: dotenv!("FIRESTORE_LOCATION").to_owned(),
        update_where: dotenv!("UPDATE_WHERE").to_owned(),
    }
});
