#![allow(clippy::needless_range_loop)]

use crate::comp::shared::team;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

use crate::comp::avg::math::YearAround;
use crate::comp::http::get;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::db::firebase::YearStore;

pub struct YearData {
    pub cache: HashMap<u16, TeamYearAroundJsonParser>,
}

impl YearData {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn check_cache(mut self, json: TeamYearAroundJsonParser, team: &str) -> (Self, bool) {
        let loc: u16 = team.parse().unwrap();
        if let Some(compare) = self.cache.get(&loc) {
            if compare == &json {
                println!("Skipping {team}, The data is not updated");
                return (self, false);
            }
        }
        self.cache.insert(loc, json);
        (self, true)
    }
    pub async fn get_new_data(team: &str, year_check: u16) -> Option<TeamYearAroundJsonParser> {
        let mut _failed: u8 = 0;
        loop {
            let response = get(team, year_check).await;
            if let Ok(json) = response {
                return Some(json);
            } else {
                if _failed == 120 {
                    println!("failed to get data");
                    return None;
                }
                _failed += 1
            }
        }
    }
    pub async fn update(mut self, year_check: u16) -> Result<Self, Self> {
        let teams = team();
        let mut wait: Vec<JoinHandle<()>> = vec![];
        let amount = teams.len() - 1;
        let mut good: bool = false;
        for loc in 0..amount + 1 {
            let team = teams[loc].to_string();
            let Some(data) =
                Self::get_new_data(&team, year_check).await else {
                return Err(self);
            };
            let mut _allow: bool = false;
            (self, _allow) = self.check_cache(data.clone(), &team);
            if _allow {
                let year = YearAround::new(data).calculate(&team);
                let Ok(year) = year else {
                    println!("failed to parse data");
                    return Err(self);
                };
                //checking if data exists
                if year.rp.avg.is_none() {
                    println!("Advanced data unavailable for team {}", &team)
                }
                if year.points.lowest == 10000 {
                    println!("Data unavailable for {} , skipping...", &team)
                } else {
                    println!(
                        "Full data is found and is pushed to firstore for {}!\n\
            Amount Completed {}/{}",
                        &team, loc, amount
                    );
                    good = true;
                    wait.push(thread::spawn(move || {
                        YearStore::new(year)
                            .set_year(&team, year_check)
                            .expect("failed when writing data");
                    }));
                }
            }
        }
        if !good {
            return Err(self);
        }
        println!("waiting for jobs to finish");
        for thread in wait {
            if thread.join().is_err() {
                return Err(self);
            }
        }
        println!("done!");
        Ok(self)
    }
}
