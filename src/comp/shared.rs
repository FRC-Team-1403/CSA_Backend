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

pub fn deviation(data: &Vec<i16>) -> f32 {
    if data.len() <= 2 {
        return 0.0;
    }
    let mut data = data.to_owned();
    data.sort();
    let half = (data.len() - 1) / 2;
    let (low, high) = data.split_at(half);
    avg(high.to_owned()) - avg(low.to_owned())
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct BreakDown {
    pub auto_points: Option<i16>,
    pub foul_points: Option<i16>,
    pub rp: Option<i16>,
    pub auto_auto_bridge_state: Option<String>,
    pub end_game_bridge_state: Option<String>,
    pub sustainability_bonus_achieved: Option<bool>,
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
                }
            }
        };
    }
    BreakDown::default()
}

pub fn avg(avg_score: Vec<i16>) -> f32 {
    avg_score.par_iter().sum::<i16>() as f32 / avg_score.len() as f32
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
