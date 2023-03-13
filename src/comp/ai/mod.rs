use crate::comp::shared::avg;
use rayon::prelude::*;
use std::collections::HashMap;

use super::avg::math::YearAround;
use super::event::math::EventData;

pub struct Ai<'a> {
    year: Option<&'a HashMap<u16, YearAround>>,
    event: Option<&'a HashMap<u16, Vec<EventData>>>,
}

impl Ai<'_> {
    fn deviation(data: &Vec<i16>) -> f32 {
        let half = (data.len() - 1) / 2;
        let low = &data[..half];
        let high = &data[half..];
        avg(high.to_owned()) - avg(low.to_owned())
    }
    pub fn calc_year_br(self) {
        let Some(year) = &self.year else {
            return;
        };
        let data: Vec<(&u16, f32)> = year
            .par_iter()
            .map(|(team, data)| {
                let penalty: f32;
                let rp: f32;
                if let Some(pen) = data.pen.avg {
                    penalty = pen;
                    rp = data.rp.avg.unwrap();
                } else {
                    penalty = 0.0;
                    rp = 0.0;
                }
                (
                    team,
                    (data.points.avg / 2.5) + (data.win_rato * 10.0) + (rp * 15.0)
                        - ((penalty * 2.0) + (Self::deviation(&data.points.graph) * 2.5)),
                )
            })
            .collect();
        for (a, b) in data {
            println!("{}, {}", a, b);
        }
    }
}
