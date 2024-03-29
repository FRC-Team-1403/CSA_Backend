#![allow(clippy::needless_late_init)]

use crate::comp::parse::{Root2, ScoreBreakdown};
use rayon::prelude::*;

pub fn get_score(team: &Team, json: Root2) -> i16 {
    match team {
        Team::Blue => json.alliances.blue.score,
        Team::Red => json.alliances.red.score,
    }
}

pub fn get_teammates(team: &Team, json: Root2) -> Vec<String> {
    let team_list;
    match team {
        Team::Blue => team_list = json.alliances.blue.team_keys,
        Team::Red => team_list = json.alliances.red.team_keys,
    }
    remove_frc(team_list)
}

pub fn remove_frc(who: Vec<String>) -> Vec<String> {
    who.iter().map(|x| x.replace("frc", "")).collect()
}

pub fn compare_highest(old: i16, new: i16) -> i16 {
    if old > new {
        return old;
    }
    new
}

pub fn compare_lowest(old: i16, new: i16) -> i16 {
    if old < new {
        return old;
    }
    new
}

pub fn deviation(numbers: &Vec<i16>) -> f32 {
    if numbers.len() <= 2 {
        return 0.0;
    }
    let mean = numbers.iter().sum::<i16>() as f32 / numbers.len() as f32;
    let variance = numbers
        .iter()
        .map(|x| (x.to_owned() as f32 - mean).powi(2))
        .sum::<f32>()
        / numbers.len() as f32;
    variance.sqrt()
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BreakDown {
    pub auto_points: Option<i16>,
    pub foul_points: Option<i16>,
    pub rp: Option<i16>,
    pub auto_auto_bridge_state: Option<String>,
    pub end_game_bridge_state: Option<String>,
    pub sustainability_bonus_achieved: Option<bool>,
    pub auto_game_piece_count: Option<i16>,
    pub auto_game_piece_points: Option<i16>,
    pub telop_game_piece_count: Option<i16>,
    pub telop_game_piece_points: Option<i16>,
}

pub fn get_breakdown_data(breakdown: Option<ScoreBreakdown>, team: &Team) -> BreakDown {
    if let Some(breakdown) = breakdown {
        return match team {
            Team::Red => {
                let location = breakdown.red;
                BreakDown {
                    auto_points: Some(location.auto_points),
                    foul_points: Some(location.foul_points),
                    rp: Some(location.rp),
                    auto_auto_bridge_state: location.auto_bridge_state,
                    end_game_bridge_state: location.end_game_bridge_state,
                    sustainability_bonus_achieved: location.sustainability_bonus_achieved,
                    auto_game_piece_count: Some(location.auto_game_piece_count),
                    auto_game_piece_points: Some(location.auto_game_piece_points),
                    telop_game_piece_count: Some(location.telop_game_piece_count),
                    telop_game_piece_points: Some(location.telop_game_piece_points),
                }
            }
            Team::Blue => {
                let location = breakdown.blue;
                BreakDown {
                    auto_points: Some(location.auto_points),
                    foul_points: Some(location.foul_points),
                    rp: Some(location.rp),
                    auto_auto_bridge_state: location.auto_bridge_state,
                    end_game_bridge_state: location.end_game_bridge_state,
                    sustainability_bonus_achieved: location.sustainability_bonus_achieved,
                    auto_game_piece_count: Some(location.auto_game_piece_count),
                    auto_game_piece_points: Some(location.auto_game_piece_points),
                    telop_game_piece_count: Some(location.telop_game_piece_count),
                    telop_game_piece_points: Some(location.telop_game_piece_points),
                }
            }
        };
    }
    BreakDown {
        auto_points: None,
        foul_points: None,
        rp: None,
        auto_auto_bridge_state: None,
        end_game_bridge_state: None,
        sustainability_bonus_achieved: None,
        auto_game_piece_count: None,
        auto_game_piece_points: None,
        telop_game_piece_count: None,
        telop_game_piece_points: None,
    }
}

pub fn avg(avg_score: Vec<i16>) -> f32 {
    avg_score.par_iter().sum::<i16>() as f32 / avg_score.len() as f32
}
pub fn avg_f32(avg_score: Vec<f32>) -> f32 {
    avg_score.par_iter().sum::<f32>() / avg_score.len() as f32
}

pub fn check_win(compare: Team, losses: i16, wins: i16, winner: &str) -> (i16, i16) {
    match compare {
        Team::Red => {
            if winner.contains("red") {
                (losses, wins + 1)
            } else {
                (losses + 1, wins)
            }
        }
        Team::Blue => {
            if winner.contains("blue") {
                (losses, wins + 1)
            } else {
                (losses + 1, wins)
            }
        }
    }
}

pub enum Team {
    Blue,
    Red,
}
