use super::avg::math::YearAround;
use super::shared::avg;
use crate::comp::shared::deviation;
use crate::ram::get_pub;
use log::debug;
use plr::regression::OptimalPLR;

pub enum Type<'a> {
    Match(&'a u16),
    Year,
}

pub struct Ai {}

impl Ai {
    fn regression(vals: &Vec<i16>) -> f64 {
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
        let last = segments.len() - 1;
        drop(segments);
        let finish = plr.finish();
        if let Some(data_p) = finish {
            let result = (data_p.slope * last as f64) + data_p.intercept;
            if result > 160.0 || result.is_sign_negative() {
                return Self::avg_regession(vals.to_owned()) as f64;
            }
            return result;
        }
        Self::avg_regession(vals.to_owned()) as f64
    }
    fn avg_regession(vals: Vec<i16>) -> f32 {
        let calc = {
            if vals.len() > 5 {
                let parse = vals.len() - 5;
                vals[parse..].to_owned()
            } else {
                vals
            }
        };
        avg(calc)
    }
    pub fn calc(match_data: &YearAround, what: Type) -> f32 {
        let match_br = Self::calc_avg_br(match_data);
        let year_br: f32 = {
            match what {
                Type::Match(team) => {
                    if let Some(year) = get_pub().get(team) {
                        Self::calc_avg_br(year)
                    } else {
                        Self::calc_avg_br(match_data)
                    }
                }
                Type::Year => Self::calc_avg_br(match_data),
            }
        };
        match_br + (year_br / 1.5)
    }
    fn math(
        avg_points: f32,
        win_ratio: f32,
        rp: f32,
        penalty: f32,
        points_graph: &Vec<i16>,
    ) -> f32 {
        let val = ((avg_points + Self::avg_regession(points_graph.to_owned()) / 2.0) / 2.5)
            + (win_ratio * 10.0)
            + (rp)
            - ((penalty / 2.0) + (deviation(points_graph) / 3.2));
        debug!("Ai val is: {}", val);
        val
    }

    fn calc_avg_br(year: &YearAround) -> f32 {
        let penalty: f32;
        let rp: f32;
        if let Some(pen) = year.pen.avg {
            penalty = pen;
        } else {
            penalty = 0.0;
        }
        if let Some(rp_data) = year.rp.avg {
            rp = rp_data;
        } else {
            rp = 0.0;
        }
        Self::math(
            year.points.avg,
            year.win_rato,
            rp,
            penalty,
            &year.points.graph,
        )
    }
}
