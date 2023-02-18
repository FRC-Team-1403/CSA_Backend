#![allow(clippy::needless_range_loop)]

use crate::comp::shared::team;
use std::collections::HashMap;

use crate::comp::avg::math::YearAround;
use crate::comp::http::get_yearly;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::db::firebase::YearStore;

const PUBLIC_CACHE: u16 = 16969;

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
    pub async fn get_new_data(what: SendType, frc: &str) -> Option<TeamYearAroundJsonParser> {
        let mut _failed: u8 = 0;
        loop {
            let response = get_yearly(&what, frc).await;
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
        let teams = team();
        let amount = teams.len() - 1;
        let mut good: bool = false;
        for loc in 0..amount + 1 {
            let team = teams[loc].to_string();
            let Some(json) =
                Self::get_new_data(what.clone(), &team).await else {
                return Err(self);
            };
            let mut _allow: bool = false;
            (self, _allow) = self.check_cache(json.clone(), &what, &teams[loc]);
            if _allow {
                match what.clone() {
                    SendType::Year(year_check) => {
                        let year = YearAround::new(json).calculate(&team);
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
                            good = true;
                            let e = YearStore::new(year).set_year(&team, year_check);
                            match e {
                                Ok(e) => {
                                    println!(
                                        "Full data is found and is pushed to firstore for {}!\n\
                                Amount Completed {}/{}\nWith Status: {}",
                                        &team, loc, amount, e
                                    );
                                }
                                Err(err) => {
                                    println!(
                                        "Failed for {}!\n\
                                Amount Completed {}/{}\n with message {}",
                                        &team, loc, amount, err
                                    );
                                }
                            }
                        }
                    }
                    SendType::Match => {
                        let calc = YearAround::new(json.clone());
                        for team in teams {
                            let team = team.to_string();
                            let team_data = calc.clone().calculate(&team);
                            let Ok(data) = team_data else {
                                return Err(self);
                            };
                        }
                        todo!();
                    }
                }
            }
        }
        if !good {
            return Err(self);
        }
        println!("waiting for jobs to finish");
        println!("done!");
        Ok(self)
    }
}

#[derive(Debug, Clone)]
pub enum SendType {
    Year(u16),
    Match,
}
