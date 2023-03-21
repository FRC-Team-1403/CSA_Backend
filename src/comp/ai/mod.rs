use super::avg::math::YearAround;
use crate::comp::shared::avg;
use crate::ram::get_pub;
use log::{debug, warn};
use plr::regression::OptimalPLR;
use std::thread;
use std::time::Duration;
mod train;
const AI_VALUE: AiValue = AiValue {
    positive_slope: 5.0,
    win_ratio: 10.0,
    ai_guess: 1.0,
    avg: 1.2,
    deviation: 2.0,
    ranking_points: 1.5,
    year_value: 0.5,
    recent: 3,
};

struct AiValue {
    win_ratio: f32,
    ai_guess: f32,
    avg: f32,
    deviation: f32,
    ranking_points: f32,
    positive_slope: f32,
    year_value: f32,
    recent: usize,
}

pub struct Ai {}

impl Ai {
    fn slope(vals: &[i16]) -> bool {
        let data_points: Vec<(f64, f64)> = vals
            .iter()
            .enumerate()
            .map(|(index, val)| (index.to_owned() as f64, val.to_owned() as f64))
            .collect();
        let mut plr = OptimalPLR::new(0.5);
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
            if vals.len() > AI_VALUE.recent + 2 {
                let parse = vals.len() - AI_VALUE.recent;
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
                    return ((Self::math_v2(year) * AI_VALUE.year_value)
                        + Self::math_v2(match_data))
                        / (AI_VALUE.year_value + 1.0);
                }
                warn!("Failed to find data for {}, waiting....", team);
                thread::sleep(Duration::from_secs(1))
            }
        }
        Self::math_v2(match_data)
    }
    pub fn calc_year(year_data: &YearAround) -> f32 {
        Self::math_v2(year_data)
    }

    fn math_v2(data: &YearAround) -> f32 {
        let rp_guess = Self::guess_next(&data.rp.graph) * AI_VALUE.ranking_points;
        let add = {
            if Self::slope(&data.points.graph) {
                AI_VALUE.positive_slope
            } else {
                0.0
            }
        };
        let ai_val = (((data.points.avg * AI_VALUE.avg)
            + (Self::guess_next(&data.points.graph) * AI_VALUE.ai_guess))
            / (AI_VALUE.ai_guess + AI_VALUE.avg))
            + (data.win_rato * AI_VALUE.win_ratio)
            + add
            + rp_guess
            - (data.deviation * AI_VALUE.deviation);
        debug!("Ai val is: {}", ai_val);
        ai_val
    }
}
