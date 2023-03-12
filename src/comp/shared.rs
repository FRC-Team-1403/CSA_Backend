#![allow(clippy::needless_late_init)]

use crate::comp::parse::{Root2, ScoreBreakdown};

pub fn get_score(team: &Team, json: Root2) -> i16 {
    match team {
        Team::Blue => json.alliances.blue.score,
        Team::Red => json.alliances.red.score,
    }
}

pub fn get_teammates(team: Team, json: Root2) -> Vec<String> {
    let team_list;
    match team {
        Team::Blue => team_list = json.alliances.blue.team_keys,
        Team::Red => team_list = json.alliances.red.team_keys,
    }
    remove_frc(team_list)
}

pub fn remove_frc(who: Vec<String>) -> Vec<String> {
    let mut return_data = vec![];
    for x in who {
        return_data.push(x.replace("frc", ""))
    }
    return_data
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

pub fn get_breakdown_data(
    breakdown: Option<ScoreBreakdown>,
    team: &Team,
) -> (
    Option<i16>,
    Option<i16>,
    Option<i16>,
    Option<String>,
    Option<String>,
    Option<bool>,
) {
    if let Some(breakdown) = breakdown {
        return match team {
            Team::Red => {
                let location = breakdown.red;
                (
                    Some(location.auto_points),
                    Some(location.foul_points),
                    Some(location.rp),
                    location.auto_bridge_state,
                    location.end_game_bridge_state,
                    location.sustainability_bonus_achieved,
                )
            }
            Team::Blue => {
                let location = breakdown.blue;
                (
                    Some(location.auto_points),
                    Some(location.foul_points),
                    Some(location.rp),
                    location.auto_bridge_state,
                    location.end_game_bridge_state,
                    location.sustainability_bonus_achieved,
                )
            }
        };
    }
    (None, None, None, None, None, None)
}

pub fn avg(avg_score: Vec<i16>) -> f32 {
    avg_score.iter().sum::<i16>() as f32 / avg_score.len() as f32
}

pub fn check_win(compare: Team, losses: i16, wins: i16, winner: &str) -> (i16, i16) {
    match compare {
        Team::Red => {
            if winner == "red" {
                (losses, wins + 1)
            } else {
                (losses + 1, wins)
            }
        }
        Team::Blue => {
            if winner == "blue" {
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
