//allow for now
#![allow(dead_code)]
extern crate core;

#[macro_use]
extern crate dotenv_codegen;

mod comp;
mod db;
mod ram;
mod server;
pub mod startup;

#[tokio::main]
async fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
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
