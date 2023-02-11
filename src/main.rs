//allow for now
#![allow(dead_code)]
extern crate core;
#[macro_use]
extern crate dotenv_codegen;

use crate::ram::ENV;

mod comp;
mod db;
mod ram;
mod server;

#[tokio::main]
async fn main() {
    let sentry_dsn = ENV.sentry_dsn.clone();
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
