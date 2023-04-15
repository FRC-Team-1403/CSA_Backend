use super::avg::math::YearAround;
use crate::comp::shared::avg;
use crate::ram::get_pub;
use log::error;
use log::{debug, warn};
use plr::regression::OptimalPLR;
use plr::Segment;
use std::thread;
use std::time::Duration;

mod train;

const AI_VALUE: AiValue = AiValue {
    plr: 0.001,
    positive_slope: 3.0,
    win_ratio: 70.0,
    ai_guess: 1.2,
    avg: 3.5,
    deviation: 1.4,
    ranking_points: 6.5,
    year_value: 5.2,
    recent: 5,
};

struct AiValue {
    plr: f32,
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
    fn line_point_regression(vals: &[i16]) -> f32 {
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
        if let Some(seg) = plr.finish() {
            ((((seg.slope as f32 * segments.len() as f32) + seg.intercept as f32) * AI_VALUE.plr)
                + Self::guess_next(vals))
                / (1.0 + AI_VALUE.plr)
                + Self::slope(&seg)
        } else if let Some(seg) = segments.last() {
            ((((seg.slope as f32 * segments.len() as f32) + seg.intercept as f32) * AI_VALUE.plr)
                + Self::guess_next(vals))
                / (1.0 + AI_VALUE.plr)
                + Self::slope(seg)
        } else {
            0.0
        }
    }
    fn slope(seg: &Segment) -> f32 {
        if seg.slope.is_sign_positive() {
            AI_VALUE.positive_slope
        } else {
            0.0
        }
    }
    fn guess_next(vals: &[i16]) -> f32 {
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
        let mut tried: u8 = 0;
        loop {
            if let Some(year) = get_pub().get(team) {
                // this allows more value to the recent data
                return ((Self::math_v2(year, Comp::Year) * AI_VALUE.year_value)
                    + Self::math_v2(match_data, Comp::Match))
                    / (AI_VALUE.year_value + 1.0);
            }
            tried += 1;
            if tried > 10 {
                error!("Dead lock or some other and it failed to get data error");
                return Self::math_v2(match_data, Comp::Match);
            }
            warn!("Failed to find data for {}, waiting....", team);
            thread::sleep(Duration::from_secs_f32(0.5))
        }
    }

    pub fn calc_year(year_data: &YearAround) -> f32 {
        Self::math_v2(year_data, Comp::Year)
    }

    fn math_v2(data: &YearAround, comp: Comp) -> f32 {
        let rp_guess = data.rp.avg.unwrap_or(0.0) * AI_VALUE.ranking_points;
        let points = match comp {
            Comp::Year => data.points.avg,
            Comp::Match => {
                ((data.points.avg * AI_VALUE.avg)
                    + (Self::line_point_regression(&data.points.graph) * AI_VALUE.ai_guess))
                    / (AI_VALUE.ai_guess + AI_VALUE.avg)
            }
        };
        let ai_val = points + (data.win_rato * AI_VALUE.win_ratio) + rp_guess
            - (data.deviation * AI_VALUE.deviation);
        debug!("Ai val is: {}", ai_val);
        ai_val
    }
}

enum Comp {
    Year,
    Match,
}
