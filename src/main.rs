//allow for now
#![allow(dead_code)]
extern crate core;

use dotenv;
use std::env;
use std::path::{Path};

// use crate::constant::SENTRY_DSN;

mod comp;
mod db;
mod server;

#[tokio::main]
async fn main() {
    let my_path = env::home_dir().map(|a| a.join("/.env")).unwrap();
    dotenv::from_path(my_path.as_path()).expect("No .env file detected");
    let sentry_dsn = dotenv::var("SENTRY_DSN").unwrap();

    let _guard = sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            enable_profiling: true,
            profiles_sample_rate: 1.0,
            ..Default::default()
        },
    ));
    server::run().await;
}
