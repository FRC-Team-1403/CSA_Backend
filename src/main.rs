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
    set_var("RUST_LOG", "info");
    env_logger::init();
    let wait = thread::spawn(|| {
        info!(
            "Teams That Will Be Tracked:\n{:?}\n\
        The Event Name: {}\n",
            ENV.teams, ENV.firestore_collection
        );
    });
    let _guard = sentry::init((
        dotenv!("SENTRY_DSN"),
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
