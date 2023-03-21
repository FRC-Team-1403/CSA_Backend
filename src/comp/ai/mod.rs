use super::avg::math::YearAround;
use crate::comp::shared::{avg, deviation};
use crate::ram::get_pub;
use log::debug;
use plr::regression::OptimalPLR;
use std::thread;
use std::time::Duration;

pub struct Ai {}

impl Ai {
    fn slope(vals: &Vec<i16>) -> bool {
        if vals.len() < 6 {}
        let data_points: Vec<(f64, f64)> = vals
            .iter()
            .enumerate()
            .map(|(index, val)| (index.to_owned() as f64, val.to_owned() as f64))
            .collect();
        let mut plr = OptimalPLR::new(0.05);
        let mut segments = Vec::new();
        for (x, y) in data_points {
            if let Some(segment) = plr.process(x, y) {
                segments.push(segment);
            }
        }
        if let Some(slope) = plr.finish() {
            return slope.slope > 0.0;
        }
        false
    }

    fn guess_next(vals: &Vec<i16>) -> f32 {
        let calc = {
            if vals.len() > 5 {
                let parse = vals.len() - 5;
                vals[parse..].to_owned()
            } else {
                vals.to_owned()
            }
        };
        avg(calc)
    }

    pub fn calc_match(match_data: &YearAround, team: &u16) -> f32 {
        if match_data.points.graph.len() < 2 {
            loop {
                if let Some(year) = get_pub().get(team) {
                    // this allows more value to the recent data
                    return (Self::math_v2(year) + (Self::math_v2(match_data) * 2.0)) / 3.0;
                }
                thread::sleep(Duration::from_secs(1))
            }
        }
        Self::math_v2(match_data)
    }
    pub fn calc_year(year_data: &YearAround) -> f32 {
        Self::math_v2(year_data)
    }

    fn math_v2(data: &YearAround) -> f32 {
        let rp_guess = Self::guess_next(&data.rp.graph) * 1.5;
        let add = {
            if Self::slope(&data.points.graph) {
                5.0
            } else {
                0.0
            }
        };
        let val = ((data.points.avg * 1.2 + Self::guess_next(&data.points.graph) / 2.2) / 2.5)
            + (data.win_rato * 10.0)
            + add
            + rp_guess
            - (data.deviation * 1.5);
        debug!("Ai val is: {}", val);
        val
    }
}
