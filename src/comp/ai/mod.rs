use crate::comp::shared::avg;
use crate::ram::get_pub;

use super::avg::math::YearAround;

pub struct Ai {}

impl Ai {
    pub fn calc(year: &YearAround, team: &u16) -> f32 {
        let match_br = Self::calc_avg_br(year);
        let year_br: f32 = {
            if let Some(year) = get_pub().get(team) {
                Self::calc_avg_br(year)
            } else {
                0.0
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
            - (penalty + (Self::deviation(points_graph) * 1.5))
    }

    fn deviation(data: &Vec<i16>) -> f32 {
        let half = (data.len() - 1) / 2;
        let low = &data[..half];
        let high = &data[half..];
        avg(high.to_owned()) - avg(low.to_owned())
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
