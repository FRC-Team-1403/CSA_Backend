#![allow(clippy::needless_range_loop)]

use crate::comp::shared::team;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

use crate::comp::avg::math::YearAround;
use crate::comp::http::get_yearly;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::db::firebase::YearStore;

const publicCashe : u16 = 696969;

pub struct YearData {
    pub cache: HashMap<u16, TeamYearAroundJsonParser>,
}

impl YearData {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn check_cache(mut self, json: TeamYearAroundJsonParser, team: &SendType) -> (Self, bool) {
        let loc: u16;
        match team {
            SendType::Year(_, team) => {
                if let Some(compare) = self.cache.get(&team.parse().unwrap()) {
                    if compare == &json {
                        println!("Skipping {loc}, The data is updated");
                        return (self, false);
                    }
                }
                loc = team.parse().unwrap();
            },
            SendType::Match => {
                if let Some(compare) = self.cache.get(&publicCashe) {
                    if compare == &json {
                        println!("Skipping match update,The data is updated");
                        return (self, false);
                    }
                }
                loc = publicCashe;
            },
        }
        self.cache.insert(loc, json);
        (self, true)
    }
    //work on this
    pub async fn get_new_data(team: &str, what: SendType) -> Option<TeamYearAroundJsonParser> {
        let mut _failed: u8 = 0;
        loop {
            let response = get_yearly(&what).await;
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
    pub async fn update(mut self, what: SendType ) -> Result<Self, Self> {
        let teams = team();
        let mut wait: Vec<JoinHandle<()>> = vec![];
        let amount = teams.len() - 1;
        let mut good: bool = false;
        for loc in 0..amount + 1 {
            let team = teams[loc].to_string();
            let Some(data) =
                Self::get_new_data(&team, what.clone()).await else {
                return Err(self);
            };
            let mut _allow: bool = false;
            (self, _allow) = self.check_cache(data, &what);
            if _allow {
                match what.clone() {
                    SendType::Year(year_check, team) => {
                        let year = YearAround::new(data::).calculate(&team);
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
                    SendType::Match => todo!(),
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


#[derive(Debug, Clone,)]
pub enum SendType {
    Year(u16, String),
    Match
}