use crate::comp::shared::deviation;
use crate::ram::get_pub;

use super::avg::math::YearAround;

pub enum Type<'a> {
    Match(&'a u16),
    Year,
}

pub struct Ai {}

impl Ai {
    pub fn calc(match_data: &YearAround, what: Type) -> f32 {
        let match_br = Self::calc_avg_br(match_data);
        let year_br: f32 = {
            match what {
                Type::Match(team) => {
                    if let Some(year) = get_pub().get(team) {
                        Self::calc_avg_br(year)
                    } else {
                        0.0
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
        (avg_points / 2.0) + (win_ratio * 10.0) + (rp * 15.0)
            - (penalty + (deviation(points_graph) * 1.5))
    }

    fn calc_avg_br(year: &YearAround) -> f32 {
        let penalty: f32;
        let rp: f32;
        if let Some(pen) = year.pen.avg {
            penalty = pen;
            rp = year.rp.avg.unwrap();
        } else {
            penalty = 0.0;
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
