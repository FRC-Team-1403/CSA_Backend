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
        613, 714, 747, 752, 816, 869, 896, 1089, 1228, 1257, 1279, 1403, 1626, 1647, 1672, 1676,
        1791, 1807, 1809, 1811, 1812, 1914, 1923, 1989, 2016, 2070, 2180, 2191, 2458, 2495, 2554,
        2577, 2590, 2600, 2720, 2722, 2729, 3142, 3151, 3314, 3340, 3515, 3637, 4035, 4281, 4361,
        4475, 4573, 4652, 4653, 4750, 5113, 5310, 5420, 5438, 5457, 5624, 5666, 5684, 5732, 5895,
        5992, 6015, 6016, 6203, 6226, 6860, 6897, 6921, 6943, 6945, 7024, 7045, 7110, 7587, 7771,
        7801, 7853, 7877, 8075, 8102, 8130, 8139, 8157, 8513, 8588, 8628, 8630, 8704, 8706, 8707,
        8714, 8721, 8771, 8801, 9015, 9064, 9100, 9116, 11, 25, 41, 56, 75, 87, 102, 136, 193, 203,
        204, 219, 223, 224, 265, 293, 303, 316, 555,
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
) -> (Option<i16>, Option<i16>, Option<i16>) {
    if let Some(breakdown) = breakdown {
        return match team {
            Team::Red => {
                let location = breakdown.red;
                (
                    Some(location.auto_points),
                    Some(location.foul_points),
                    Some(location.rp),
                )
            }
            Team::Blue => {
                let location = breakdown.red;
                (
                    Some(location.auto_points),
                    Some(location.foul_points),
                    Some(location.rp),
                )
            }
        };
    }
    (None, None, None)
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
