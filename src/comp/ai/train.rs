#![allow(unused_imports)]

use log::{error, info, warn};
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
use crate::comp::shared::{avg_f32, deviation};
use rand::prelude::*;

const START_FROM: usize = 25;
const YEAR: u16 = 2023;
const GAMES: u8 = 20;
#[test]
fn train() {
    set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Ai Test running");
    let loop_keys = get_keys();
    let train_final: Vec<f32> = loop_keys
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
            let calc_teams = Tba::get_teams(key, &ENV.api_key).unwrap();
            calc_teams.par_iter().for_each(|team| {
                let json = get_yearly(&SendType::Year(2023), &team.to_string(), key).unwrap();
                let year = YearAround::new(json).calculate(&team.to_string());
                get_pub().insert(*team, year.unwrap());
            });
            let train_results: Vec<f32> = matches
                .par_iter()
                .map(|location| {
                    let (train, predict) = api_data.split_at(location.to_owned());
                    if predict.is_empty() || train.is_empty() {
                        panic!(
                            "Bad data in the vector\n train data set : {:?}\n predict data set : {:?}\n",
                            predict, train
                        );
                    }
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
                        100.0
                    } else if winner_ai == "blue" {
                        let ratio = ((blue_br / (red_br + blue_br)) * 100.0) as f32;
                        warn!("AI WRONG, ratio by ai: {ratio}",);
                        100.0 - ratio
                    } else {
                        let ratio = ((red_br / (red_br + blue_br)) * 100.0) as f32;
                        warn!("AI WRONG, ratio by ai: {ratio}",);
                        100.0 - ratio
                    }
                })
                .collect();
            avg_f32(train_results)
        })
        .collect();
    let avg = avg_f32(train_final);
    info!("Ai Score: {} ", avg);
    if avg < 76.0 {
        panic!(
            "Ai test failed with different score\n the ai score is: {}",
            avg
        )
    }
}

pub fn get_keys() -> Vec<String> {
    let response = reqwest::blocking::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/events/{YEAR}/keys",
        ))
        .header("X-TBA-Auth-Key", &ENV.api_key)
        .send()
        .unwrap();
    let keys = response.json::<Vec<String>>().unwrap();
    if GAMES == 0 {
        return keys;
    }
    let (_, last) = keys.split_at(GAMES as usize - keys.len());
    last.to_owned()
}

use crate::comp::parse::TeamYearAroundJsonParser;
use crate::ram::get_pub;
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
