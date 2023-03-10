use crate::ram::{CACHE_MATCH, CACHE_MATCH_AVG};
use rayon::prelude::*;
use std::future::pending;

pub struct Ai {}

impl Ai {
    pub fn new() -> Self {
        Ai {}
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
                if let Ok(pen) = data.pen.avg {
                    penalty = pen;
                    rp = data.rp.avg.unwrap();
                } else {
                    penalty = 0.0;
                    rp = 0.0;
                }
                (
                    team,
                    (data.points.avg / 2.5) + (data.win_rato * 100.0) + (rp * 15.0)
                        - (penalty * 2.0) / 2,
                )
            })
            .collect();
        drop(year);
        // let Ok(matchd) = CACHE_MATCH_AVG.try_lock() else {
        //     return;
        // };
        for (a, b) in data {
            println!("{}, {}", a, b);
        }
    }
}
