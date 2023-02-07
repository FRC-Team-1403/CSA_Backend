//allow for now
#![allow(dead_code)]
extern crate core;

use crate::constant::SENTRY_DSN;

mod constant;
mod db;
mod http;
mod server;

#[tokio::main]
async fn main() {
    let _guard = sentry::init((SENTRY_DSN, sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));
    server::run().await;
}
