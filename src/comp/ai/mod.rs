use crate::comp::shared::avg;
use crate::ram::{CACHE_MATCH, CACHE_MATCH_AVG};
use rayon::prelude::*;

pub struct Ai {}

impl Ai {
    pub fn new() -> Self {
        Ai {}
    }
    fn deviation(data: Vec<i16>) -> f32 {
        let half = (data.len() - 1) / 2;
        let low = &data[..half];
        let high = &data[half..];
        avg(high.to_owned()) - avg(low.to_owned())
    }
    pub fn calc(&mut self) {
        let Ok(year) = CACHE_MATCH_AVG.try_lock() else {
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
                    (data.points.avg / 2.5) + (data.win_rato * 100.0) + (rp * 15.0)
                        - (penalty * 2.0) / 2.0,
                )
            })
            .collect();
        // let Ok(matchd) = CACHE_MATCH_AVG.try_lock() else {
        //     return;
        // };
        for (a, b) in data {
            println!("{}, {}", a, b);
        }
    }
}
