use std::{thread, time::Duration};

use rayon::prelude::*;

use crate::{
    comp::{
        ai::Ai,
        avg::{math::YearAround, year_around_main::YearData},
        shared::avg,
    },
    ram::ENV,
};

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
            let (train, predict) = api_data.split_at(thread_rng().gen_range(3..api_data.len() - 1));
            if predict.is_empty() || train.is_empty() {
                panic!(
                    "Bad data in the vector\n train data set : {:?}\n predict data set : {:?}\n",
                    predict, train
                );
            }
            let train = YearAround::new(train.to_owned());
            let teams_br: Vec<(u16, f32)> = ENV
                .teams
                .par_iter()
                .map(|team| {
                    let ai_data = Ai::calc_match(&train, team);
                    (team.to_owned(), ai_data)
                })
                .collect();
            let predict = predict.first().unwrap().to_owned();
            let red = predict.alliances.red.team_keys;
            let blue = predict.alliances.blue.team_keys;
            let red_br = avg_br(red, &teams_br);
            let blue_br = avg_br(blue, &teams_br);
            drop(teams_br);
            let winner = predict.winning_alliance;
            let winner_ai = {
                if red_br < blue_br {
                    "blue"
                } else {
                    "red"
                }
            };
            todo!();
        })
        .collect();
    let avg = avg(train_results);
    if avg < 0.90 {
        panic!(
            "Ai test failed with a score less then 90\n the ai score is: {}",
            avg
        )
    }
}

fn avg_br(teams: Vec<String>, br_data: &[(u16, f32)]) -> f32 {
    avg(teams
        .par_iter()
        .map(|team| {
            let (team, br) = br_data
                .iter()
                .find(|(team_num, _)| team == &format!("frc{}", team_num))
                .unwrap();
            println!("Team {} br is: {}", team, br);
            br.round() as i16
        })
        .collect())
}
