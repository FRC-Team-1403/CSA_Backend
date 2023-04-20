use super::avg::math::YearAround;
use crate::comp::ai::predict_score::SCORE_AI;
use crate::comp::shared::avg;
use crate::ram::get_pub;
use log::error;
use log::{debug, warn};
use plr::regression::OptimalPLR;
use plr::Segment;
use std::thread;
use std::time::Duration;

mod predict_score;
mod train;

const AI_VALUE: AiValue = AiValue {
    plr: 0.001,
    positive_slope: 3.0,
    win_ratio: 70.0,
    ai_guess: 1.2,
    avg: 3.5,
    deviation: 1.4,
    ranking_points: 4.5,
    year_value: 5.8,
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

pub enum Math {
    Score,
    EkamAi,
}

impl Ai {
    pub fn geometic_mean(n: &[i16]) -> f32 {
        let product: f64 = n.iter().fold(1.0, |mut old_value: f64, new| {
            if old_value == 0.0 {
                old_value = 1.0
            }
            let new = if new <= &0 {
                1.0
            } else {
                new.to_owned() as f64
            };
            old_value * new
        });
        product.powf(1.0 / n.len() as f64) as f32
    }
    pub fn harmonic_mean(n: &[i16]) -> f32 {
        let reciprocal: f32 = n.iter().fold(0.0, |old_value: f32, new| {
            let new = if new <= &0 {
                1.0
            } else {
                new.to_owned() as f32
            };
            old_value + (1.0 / new)
        });
        n.len() as f32 / reciprocal
    }

    fn line_point_regression(vals: &[i16], what: Math) -> f32 {
        let muli_plr = match what {
            Math::Score => SCORE_AI.plr,
            Math::EkamAi => AI_VALUE.plr,
        };
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
            ((((seg.slope as f32 * segments.len() as f32) + seg.intercept as f32) * muli_plr)
                + Self::guess_next(vals))
                / (1.0 + muli_plr)
                + Self::slope(&seg, what)
        } else if let Some(seg) = segments.last() {
            ((((seg.slope as f32 * segments.len() as f32) + seg.intercept as f32) * muli_plr)
                + Self::guess_next(vals))
                / (1.0 + muli_plr)
                + Self::slope(seg, what)
        } else {
            0.0
        }
    }
    fn slope(seg: &Segment, what: Math) -> f32 {
        if let Math::EkamAi = what {
            if seg.slope.is_sign_positive() {
                AI_VALUE.positive_slope
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
    fn guess_next(vals: &[i16]) -> f32 {
        let calc = {
            if vals.len() > AI_VALUE.recent + 2 {
                let parse = vals.len() - AI_VALUE.recent;
                &vals[parse..]
            } else {
                vals
            }
        };
        avg(calc.to_owned())
    }
    fn get_lock_find(team: &u16) -> Option<YearAround> {
        let mut tried: u8 = 0;
        loop {
            if let Some(year) = get_pub().get(team) {
                // this allows more value to the recent data
                return Some(year.to_owned());
            }
            tried += 1;
            if tried > 10 {
                error!("Dead lock or some other and it failed to get data error");
                return None;
            }
            warn!("Failed to find data for {}, waiting....", team);
            thread::sleep(Duration::from_secs_f32(0.5))
        }
    }
    pub fn calc_match(match_data: &YearAround, team: &u16) -> f32 {
        if let Some(year) = Self::get_lock_find(team) {
            // this allows more value to the recent data
            ((Self::math_v2(&year, Comp::Year) * AI_VALUE.year_value)
                + Self::math_v2(match_data, Comp::Match))
                / (AI_VALUE.year_value + 1.0)
        } else {
            error!("Dead lock or some other and it failed to get data error");
            Self::math_v2(match_data, Comp::Match)
        }
    }

    pub fn calc_year(year_data: &YearAround) -> f32 {
        Self::math_v2(year_data, Comp::Year)
    }

    fn math_v2(data: &YearAround, comp: Comp) -> f32 {
        let rp_guess = data.rp.avg.unwrap_or(0.0) * AI_VALUE.ranking_points;
        let points = match comp {
            Comp::Year => Self::geometic_mean(&data.points.graph),
            Comp::Match => {
                ((Self::geometic_mean(&data.points.graph) * AI_VALUE.avg)
                    + (Self::line_point_regression(&data.points.graph, Math::EkamAi)
                        * AI_VALUE.ai_guess))
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
