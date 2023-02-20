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
            info!("Updating year value: ");
            year = update(year, SendType::Match).await;
            year = update(year, SendType::Year(2023)).await;
            event = event.update_match_data().await;
            transaction.finish();
            event = event.update_match_data().await;
            thread::sleep(Duration::from_secs(360))
        }
    });
    join.await.unwrap();
}

async fn update(mut year: YearData, what: SendType) -> YearData {
    let mut error: u8 = 0;
    loop {
        let tx_ctx = sentry::TransactionContext::new("Update Year", "Running from update()");
        let transaction = sentry::start_transaction(tx_ctx);
        match year.update(what.clone()).await {
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
