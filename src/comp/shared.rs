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
pub fn team() -> Vec<u16> {
    vec![
        613, 747, 752, 816, 869, 896, 1228, 1279, 1647, 1672, 1676, 1791, 1809, 1914, 1923, 1989,
        2070, 2180, 2458, 2577, 2600, 2720, 2722, 2729, 3142, 3151, 3340, 3515, 3637, 4035, 4281,
        4361, 4475, 4573, 4750, 5113, 5310, 5420, 5438, 5457, 5732, 5895, 5992, 6015, 6016, 6203,
        6226, 6860, 6921, 6943, 6945, 7024, 7045, 7110, 7771, 7801, 7853, 7877, 8102, 8139, 8157,
        8513, 8588, 8628, 8630, 8706, 8721, 8771, 8801, 9064, 9100, 11, 25, 41, 56, 75, 87, 102,
        136, 193, 203, 204, 219, 223, 224, 265, 555, 58, 88, 125, 157, 238, 246, 501, 509, 1073,
        1721, 2370, 2713, 3467, 4041, 6153, 8724, //delete later
        1574, 1576, 1577, 1690, 1943, 2096, 2230, 2630, 3065, 3083, 3211, 3388, 4319, 4338, 4661,
        5135, 5554, 5654, 5990, 6168, 6738, 6740, 6741, 7067, 9304,
    ]
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
                    Some(location.auto_bridge_state),
                    Some(location.end_game_bridge_state),
                    Some(location.sustainability_bonus_achieved),
                )
            }
            Team::Blue => {
                let location = breakdown.blue;
                (
                    Some(location.auto_points),
                    Some(location.foul_points),
                    Some(location.rp),
                    Some(location.auto_bridge_state),
                    Some(location.end_game_bridge_state),
                    Some(location.sustainability_bonus_achieved),
                )
            }
        };
    }
    (None, None, None, None, None, None)
}
pub fn avg(avg_score: Vec<i16>) -> f32 {
    let divider = avg_score.len();
    let mut divide = 0;
    for num in avg_score {
        divide += num;
    }
    divide as f32 / divider as f32
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
