#![allow(clippy::needless_late_init)]

use crate::comp::shared::team;
use crate::ram::ENV;
use std::collections::HashMap;
use std::thread;

use crate::comp::avg::math::YearAround;
use crate::comp::http::get_yearly;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::db::firebase::YearStore;
use rayon::prelude::*;

const PUBLIC_CACHE: u16 = 16969;

#[derive(Clone)]
pub struct YearData {
    pub cache: HashMap<u16, TeamYearAroundJsonParser>,
}

impl YearData {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    fn check_cache(
        mut self,
        json: TeamYearAroundJsonParser,
        what: &SendType,
        team: &u16,
    ) -> (Self, bool) {
        let loc: u16;
        match what {
            SendType::Year(_) => {
                if let Some(compare) = self.cache.get(team) {
                    if compare == &json {
                        println!("Skipping {team}, The data is updated");
                        return (self, false);
                    }
                }
                loc = *team;
            }
            SendType::Match => {
                if let Some(compare) = self.cache.get(&PUBLIC_CACHE) {
                    if compare == &json {
                        println!("Skipping match update,The data is updated");
                        return (self, false);
                    }
                }
                loc = PUBLIC_CACHE;
            }
        }
        self.cache.insert(loc, json);
        (self, true)
    }
    //work on this
    pub fn get_new_data(what: SendType, frc: &str) -> Option<TeamYearAroundJsonParser> {
        let mut _failed: u8 = 0;
        loop {
            let response = get_yearly(&what, frc);
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
    pub async fn update(mut self, what: SendType) -> Result<Self, Self> {
        match what {
            SendType::Year(_) => {
                for team_num in team() {
                    let team = team_num.to_string();
                    let Some(json) =
                        Self::get_new_data(what.clone(), &team) else {
                        return Err(self);
                    };
                    let mut _allow: bool = false;
                    (self, _allow) = self.check_cache(json.clone(), &what, &team_num);
                    if _allow {
                        let year = YearAround::new(json).calculate(&team);
                        let Ok(year) = year else {
                            println!("failed to parse data");
                            return Err(self);
                        };
                        thread::spawn(move || {
                            send_and_check(year, team);
                        });
                    }
                }
                Ok(self)
            }
            SendType::Match => {
                let Some(json) =
                    Self::get_new_data(what.clone(), "69") else {
                    return Err(self);
                };
                let mut _allow = false;
                (self, _allow) = self.check_cache(json.clone(), &what, &69);
                if _allow {
                    let calc = YearAround::new(json);
                    team().par_iter().try_for_each(|team| -> Result<(), Self> {
                        let team_calc = calc.clone();
                        let team = team.to_string();
                        let year = team_calc.calculate(&team);
                        let Ok(year) = year else {
                                return Err(self.clone());
                            };
                        thread::spawn(move || {
                            send_and_check(year, team);
                        });
                        Ok(())
                    })?;
                }
                Ok(self)
            }
        }
    }
}

fn send_and_check(year: YearAround, team: String) {
    if year.rp.avg.is_none() {
        println!("Advanced data unavailable for team {}", &team)
    }
    if year.points.lowest == 10000 {
        println!("Data unavailable for {} , skipping...", &team)
    } else {
        let e = YearStore::new(year).set_year(&team, &ENV.firestore_collection);
        match e {
            Ok(e) => {
                println!(
                    "Full data is found and is pushed to firstore for {}!\n\
                                \nWith Status: {}",
                    &team, e
                );
            }
            Err(err) => {
                println!(
                    "Failed for {}!\n\
                    with message {}",
                    &team, err
                );
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SendType {
    Year(u16),
    Match,
}
