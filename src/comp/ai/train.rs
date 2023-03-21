use std::{thread, time::Duration};

use rayon::prelude::*;

use crate::{comp::shared::avg, ram::ENV};

use rand::prelude::*;

#[test]
fn init() {
    dbg!(&ENV.teams);
}

#[tokio::test]
async fn train() {
    thread::sleep(Duration::from_secs(3));
    let api_data = crate::comp::http::get_match().await.unwrap();
    //data is recived, time to test
    let train_results: Vec<i16> = vec![0; 100]
        .par_iter()
        .filter_map(|_| {
            let (train, predict) = api_data.split_at(thread_rng().gen_range(3..api_data.len()));
            let teams_br: Vec<(u16, f32)> = vec![];
            todo!()
        })
        .collect();
    let avg = avg(train_results);
    if avg < 90.0 {
        panic!(
            "Ai test failed with a score less then 90\n the ai score is: {}",
            avg
        )
    }
}
