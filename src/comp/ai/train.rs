use std::{thread, time::Duration};

use rayon::prelude::*;

use crate::ram::ENV;

#[test]
fn init() {
    dbg!(&ENV.teams);
}

#[tokio::test]
async fn train() {
    thread::sleep(Duration::from_secs(3));
    let api_data = crate::comp::http::get_match().await.unwrap();
    //data is recived, time to test
    let train_results: Vec<f32> = vec![0; 10000].par_iter().filter_map(|_| None).collect();
}
