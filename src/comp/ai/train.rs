#![allow(unused_imports)]

use log::{info, warn};
use std::env::set_var;
use std::{thread, time::Duration};

use rayon::prelude::*;

use crate::startup::tba::Tba;
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

const START_FROM: usize = 25;

#[test]
fn train() {
    set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Ai Test running");
    let loop_keys = vec![
        "2023njrob",
        "2023ncwak",
        "2023marea",
        "2023nyro",
        "2023casf",
        "2023isde4",
        "2023vaale",
        "2023okok",
        "2023ilch",
        "2023milan",
        "2023misjo",
        "2023txbel",
        "2023orwil",
        "2023cafr",
        "2023cave",
        "2023ausc",
        "2023ncjoh",
        "2023txcha",
        "2023scand",
        "2023inpri",
        "2023njfla",
        "2023ncmec",
    ];
    let train_final: Vec<i16> = loop_keys
        .par_iter()
        .map(|key| {
            let api_data = get_yearly(&SendType::Match, "", key).unwrap();
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
                    let calc_teams = Tba::get_teams(key, &ENV.api_key).unwrap();
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
            avg(train_results) as i16
        })
        .collect();
    let avg = avg(train_final);
    info!("Ai Score: {} ", avg);
    if avg < 80.22951 {
        panic!(
            "Ai test failed with different score\n the ai score is: {}",
            avg
        )
    }
}

use crate::comp::parse::TeamYearAroundJsonParser;
use reqwest::Error;

pub fn get_yearly(
    year: &SendType,
    team: &str,
    api_key: &str,
) -> Result<TeamYearAroundJsonParser, Error> {
    let send_url: String = {
        match year {
            SendType::Year(year) => {
                format!("https://www.thebluealliance.com/api/v3/team/frc{team}/matches/{year}")
            }
            SendType::Match => {
                format!("https://www.thebluealliance.com/api/v3/event/{api_key}/matches")
            }
        }
    };
    let response = reqwest::blocking::Client::new()
        .get(send_url)
        .header("X-TBA-Auth-Key", &ENV.api_key)
        .send()?;
    response.json::<TeamYearAroundJsonParser>()
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
