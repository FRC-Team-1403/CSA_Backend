//allow for now
#![allow(dead_code)]
extern crate core;

use crate::constant::SENTRY_DSN;
use crate::http::event::Event;

mod config;
mod constant;
mod db;
mod http;
mod server;

#[tokio::main]
async fn main() {
    Event::new().update_match_data().await;
    todo!();
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
