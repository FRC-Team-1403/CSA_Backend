use std::thread;
use std::time::Duration;
use log::{info, warn};

use crate::http::year_around::year_around_main::YearData;

pub async fn run() {
    let join = tokio::spawn(async move {
        let mut year = YearData::new();
        loop {
            info!("Updating year value: {}", year);
            year = update(year).await;
            thread::sleep(Duration::from_secs(360))
        }
    });
    join.await.unwrap();
}

async fn update(mut year: YearData) -> YearData {
    let mut error: u8 = 0;
    loop {
        match year.update(2022).await {
            Ok(e) => {
                return e;
            }
            Err(e) => {
                year = e;
                error += 1;
            }
        }
        if error > 120 {
            warn!("critical Failure, skipping");
            return year;
        }
    }
}
