//allow for now
#![allow(dead_code)]
extern crate core;

use crate::constant::SENTRY_DSN;

mod constant;
mod db;
mod http;
mod server;
mod config;

#[tokio::main]
async fn main() {
    let _guard = sentry::init((
        SENTRY_DSN,
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
