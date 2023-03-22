#![allow(unused_imports)]

use log::{info, warn};
use std::env::set_var;
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

use crate::comp::avg::year_around_main::SendType;
use crate::comp::shared::deviation;
use rand::prelude::*;

const START_FROM: usize = 15;

#[test]
fn train() {
    set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Ai Test running");
    let api_data = crate::comp::http::get_yearly(&SendType::Match, "").unwrap();
    let matches: Vec<usize> = vec![0; api_data.len()]
        .iter()
        .enumerate()
        .filter_map(|(index, _)| {
            if index > START_FROM {
                Some(index)
            } else {
                None
            }
        })
        .collect();
    //data is received, time to test
    let train_results: Vec<i16> = matches
        .par_iter()
        .map(|location| {
            let (train, predict) = api_data.split_at(location.to_owned());
            if predict.is_empty() || train.is_empty() {
                panic!(
                    "Bad data in the vector\n train data set : {:?}\n predict data set : {:?}\n",
                    predict, train
                );
            }
            let calc_teams = ENV.teams.clone();
            let teams_br: Vec<(u16, f32)> = calc_teams
                .par_iter()
                .map(|team| {
                    let train = YearAround::new(train.to_owned())
                        .calculate(&team.to_string())
                        .unwrap();
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
            if winner == winner_ai {
                info!("AI passed!, blue br {}, red br {}", red_br, blue_br);
                100
            } else {
                warn!("AI WRONG, blue br {}, red br {}", red_br, blue_br);
                if winner_ai == "blue" {
                    100 - ((blue_br / (red_br + blue_br)) * 100.0) as i16
                } else {
                    100 - ((red_br / (red_br + blue_br)) * 100.0) as i16
                }
            }
        })
        .collect();
    let avg = avg(train_results);
    info!("Ai Score: {} ", avg);
    if avg < 80.22951 {
        panic!(
            "Ai test failed with different score\n the ai score is: {}",
            avg
        )
    }
}

fn avg_br(teams: Vec<String>, br_data: &[(u16, f32)]) -> f32 {
    avg(teams
        .par_iter()
        .map(|team| {
            let (_, br) = br_data
                .iter()
                .find(|(team_num, _)| team == &format!("frc{}", team_num))
                .unwrap();
            br.to_owned() as i16
        })
        .collect())
}
