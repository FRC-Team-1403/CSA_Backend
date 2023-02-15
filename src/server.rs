use std::any::Any;
use crate::comp::event::Event;
use log::{info, warn};
use std::thread;
use std::time::Duration;

use crate::comp::avg::year_around_main::{SendType, YearData};

pub async fn run() {
    let join = tokio::spawn(async move {
        let mut year = YearData::new();
        let mut event = Event::new();
        loop {
            let tx_ctx =
                sentry::TransactionContext::new("Updating new Year Value", "run() function");
            let transaction = sentry::start_transaction(tx_ctx);
            event = event.update_match_data().await;

            info!("Updating year value: ");
            year = update(year).await;
            transaction.finish();
            thread::sleep(Duration::from_secs(360))
        }
    });
    join.await.unwrap();
}

async fn update(mut year: YearData) -> YearData {
    let mut error: u8 = 0;
    loop {
        let tx_ctx = sentry::TransactionContext::new("Update Year", "Running from update()");
        let transaction = sentry::start_transaction(tx_ctx);
        match year.update(SendType::Year(2022, "88".to_owned())).await {
            Ok(e) => {
                return e;
            }
            Err(e) => {
                year = e;
                error += 1;
            }
        }
        transaction.finish();
        if error > 120 {
            warn!("critical Failure, skipping");
            return year;
        }
    }
}
