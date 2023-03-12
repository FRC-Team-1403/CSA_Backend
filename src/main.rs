//allow for now
#![allow(dead_code)]
extern crate core;

#[macro_use]
extern crate dotenv_codegen;

use crate::ram::ENV;
use log::info;
use std::env::set_var;
use std::thread;

mod comp;
mod db;
mod ram;
mod server;
pub mod startup;

#[tokio::main]
async fn main() {
    let wait = thread::spawn(|| {
        info!(
            "\nTeams That Will Be Tracked:\n{:?}\n\
        The Event Name: {}\n",
            ENV.teams, ENV.firestore_collection
        );
    });
    set_var("RUST_LOG", "info");
    env_logger::init();
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
    wait.join().unwrap();
    server::run().await;
}
