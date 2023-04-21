#![allow(unused_imports)]

use log::{error, info, warn};
use rayon::prelude::*;
use std::env::set_var;
use std::fs::File;
use std::io::prelude::*;
use std::{fs, thread, time::Duration};

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
const GAMES: u8 = 15;

#[test]
fn train() {
    set_var("RUST_LOG", "info");
    env_logger::init();
    info!("Ai Test running");
    let loop_keys = get_keys();
    let train_final: Vec<(i16, i16)> = loop_keys
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
            //Year Data is set time to test
            let train_results: Vec<(f32, f32)> = matches
                .par_iter()
                .map(|location| {
                    let (train, predict) = api_data.split_at(location.to_owned());
                    if predict.is_empty() || train.is_empty() {
                        panic!(
                            "Bad data in the vector\n train data set : {:?}\n predict data set : {:?}\n",
                            predict, train
                        );
                    }
                    let teams_br: Vec<(u16, f32, f32)> = calc_teams
                        .par_iter()
                        .map(|team| {
                            let train = YearAround::new(train.to_owned())
                                .calculate(&team.to_string())
                                .unwrap();
                            let ai_data = Ai::calc_match(&train, team);
                            let pred_score = Ai::predict_match_score(&train, team);
                            (team.to_owned(), ai_data, pred_score)
                        })
                        .collect();
                    let predict: Root2 = predict.first().unwrap().to_owned();
                    let red = predict.alliances.red.team_keys;
                    let blue = predict.alliances.blue.team_keys;
                    let (red_br, red_score) = avg_ai_values(red, &teams_br);
                    let (blue_br, blue_score) = avg_ai_values(blue, &teams_br);
                    drop(teams_br);
                    let winner = predict.winning_alliance;
                    let winner_ai = {
                        if red_br < blue_br {
                            "blue"
                        } else {
                            "red"
                        }
                    };
                    //score predict
                    let red_diff = (red_score - predict.alliances.red.score as f32).abs();
                    let blue_diff = (blue_score - predict.alliances.blue.score as f32).abs();
                    let avg_diff = (red_diff + blue_diff) / 2.0;
                    info!("\nDiff of score for red is {red_diff}, with blue {blue_diff}\nRed Predict Score {red_score} with real score {}\nBlue Predict Score {blue_score}, with real score {}", predict.alliances.red.score, predict.alliances.blue.score);
                    if winner == winner_ai {
                        info!("AI passed!, blue br {}, red br {}", red_br, blue_br);
                        (100.0, avg_diff)
                    } else if winner_ai == "blue" {
                        let ratio = (blue_br / (red_br + blue_br)) * 100.0;
                        warn!("AI WRONG, ratio by ai: {ratio}",);
                        (100.0 - ratio, avg_diff)
                    } else {
                        let ratio = (red_br / (red_br + blue_br)) * 100.0;
                        warn!("AI WRONG, ratio by ai: {ratio}",);
                        (100.0 - ratio, avg_diff)
                    }
                })
                .collect();
            let ai_score = train_results.iter().map(|(ai, _)| ai.to_owned()).collect();
            let score_diff = train_results.iter().map(|(_, score)| score.to_owned()).collect();
            (avg_f32(ai_score) as i16, avg_f32(score_diff) as i16)
        })
        .collect();
    let ai_score = train_final.iter().map(|(ai, _)| ai.to_owned()).collect();
    let score_diff = train_final
        .iter()
        .map(|(_, score)| score.to_owned())
        .collect();
    let avg_ai = avg(ai_score);
    let score = avg(score_diff);
    info!(
        "Ai Is correct: {avg_ai}\nThe Team Score Predicter inaccuratcy +-{}",
        score / 3.0
    );
    if avg_ai < 76.0 {
        panic!(
            "Ai test failed with different score\n the ai score is: {}",
            avg_ai
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
    let (first, _) = keys.split_at(GAMES as usize);
    first.to_owned()
    // first.par_iter().for_each(|key| {
    //     info!(
    //         "{}",
    //         Tba::get_event(key, &ENV.api_key).expect("Failed While Getting Teams"),
    //     );
    // });
}

use crate::comp::parse::{Root2, TeamYearAroundJsonParser};
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
    let read_cache = send_url.replace('/', "\\");
    let read_cache = format!("src/comp/ai/cache/{read_cache}.json");
    let cache = fs::read_to_string(&read_cache);
    let what = match cache {
        Ok(e) => e,
        Err(_) => {
            let response = reqwest::blocking::Client::new()
                .get(send_url)
                .header("X-TBA-Auth-Key", &ENV.api_key)
                .send()?;
            let text = response.text()?;
            fs::write(&read_cache, &text).unwrap();
            text
        }
    };
    Ok(serde_json::from_str(&what).unwrap())
}

fn avg_ai_values(teams: Vec<String>, br_data: &[(u16, f32, f32)]) -> (f32, f32) {
    (
        teams.iter().fold(0.0, |old: f32, team| {
            let (_, br, _) = br_data
                .iter()
                .find(|(team_num, _, _)| team == &format!("frc{}", team_num))
                .unwrap();
            old + br
        }),
        teams.iter().fold(0.0, |old: f32, team| {
            let (_, _, pred_score) = br_data
                .iter()
                .find(|(team_num, _, _)| team == &format!("frc{}", team_num))
                .unwrap();
            pred_score + old
        }),
    )
}
