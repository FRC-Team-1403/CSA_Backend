use crate::charts::{populate, Version};
use crate::comp::avg::year_around_main::{SendType, YearData};
use crate::comp::event::Event;
use log::{error, info, warn};
use std::thread;
use std::time::Duration;

pub async fn run() {
    info!("Data Is Ready, Starting Server");
    update_year(SendType::Year(2023));
    thread::sleep(Duration::from_secs(120));
    update_year(SendType::Match);
    tokio::spawn(async {
        loop {
            populate(Version::Match).await;
            thread::sleep(Duration::from_secs(600));
        }
    });
    let mut event = Event::new();
    loop {
        info!("Updating Match value: ");
        event = event.update_match_data().await;
        event.updated = wait(event.updated, 5, 160);
    }
}

fn update_year(what: SendType) {
    thread::spawn(move || {
        let mut year = YearData::new();
        loop {
            let tx_ctx =
                sentry::TransactionContext::new("Updating new Year Value", "run() function");
            let transaction = sentry::start_transaction(tx_ctx);
            info!("Updating year value: ");
            year = update(year, what.clone());
            transaction.finish();
            year.updated = wait(year.updated, 15, 180);
        }
    });
}

fn update(mut year: YearData, what: SendType) -> YearData {
    let mut error: u8 = 0;
    loop {
        let tx_ctx = sentry::TransactionContext::new("Update Year", "Running from update()");
        let transaction = sentry::start_transaction(tx_ctx);
        match year.update(what.clone()) {
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
            error!("critical Failure, skipping");
            return year;
        }
    }
}

fn wait(done_before: bool, wait: u8, wait_long: u16) -> bool {
    if !done_before {
        thread::sleep(Duration::from_secs(wait as u64));
        info!("Retrying Polling");
        done_before
    } else {
        warn!("Sleeping For a Long Time");
        thread::sleep(Duration::from_secs(wait_long as u64));
        false
    }
}
