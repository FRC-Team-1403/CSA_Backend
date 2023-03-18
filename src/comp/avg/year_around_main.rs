#![allow(clippy::needless_late_init)]

use crate::comp::ai::{Ai, Type};
use crate::comp::avg::math::YearAround;
use crate::comp::http::get_yearly;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::db::firebase::YearStore;
use crate::ram::{get_pub, CACHE_MATCH_AVG, ENV};
use log::{error, info, warn};
use rayon::prelude::*;
use std::collections::HashMap;
use std::thread;

const PUBLIC_CACHE: u16 = 16969;

#[derive(Clone)]
pub struct YearData {
    pub cache: HashMap<u16, TeamYearAroundJsonParser>,
    pub updated: bool,
}

impl YearData {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            updated: false,
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
                        info!("Skipping {team}, The data is updated");
                        return (self, false);
                    }
                }
                loc = *team;
            }
            SendType::Match => {
                if let Some(compare) = self.cache.get(&PUBLIC_CACHE) {
                    if compare == &json {
                        info!("Skipping match update,The data is updated");
                        return (self, false);
                    }
                }
                loc = PUBLIC_CACHE;
            }
        }
        self.updated = true;
        self.cache.insert(loc, json);
        (self, true)
    }
    pub fn get_new_data(what: SendType, frc: &str) -> Option<TeamYearAroundJsonParser> {
        let mut _failed: u8 = 0;
        loop {
            let response = get_yearly(&what, frc);
            if let Ok(json) = response {
                return Some(json);
            } else {
                if _failed == 120 {
                    info!("failed to get data");
                    return None;
                }
                _failed += 1
            }
        }
    }
    pub fn update(mut self, what: SendType) -> Result<Self, Self> {
        match what {
            SendType::Year(year_check) => {
                for team_num in ENV.teams.clone() {
                    let team = team_num.to_string();
                    let Some(json) =
                        Self::get_new_data(what.clone(), &team) else {
                        return Err(self);
                    };
                    let mut _allow: bool = false;
                    (self, _allow) = self.check_cache(json.clone(), &what, &team_num);
                    if _allow {
                        let year = YearAround::new(json).calculate(&team);
                        let Ok(mut year) = year else {
                            error!("failed to parse data");
                            return Err(self);
                        };
                        year.br = Ai::calc(&year, Type::Year);
                        get_pub().insert(team_num, year.clone());
                        send_and_check(year, team, year_check.to_string());
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
                    ENV.teams
                        .par_iter()
                        .try_for_each(|team_num| -> Result<(), Self> {
                            let team_calc = calc.clone();
                            let team = team_num.to_string();
                            let year = team_calc.calculate(&team);
                            let year = loop {
                                if let Ok(mut year) = year {
                                    year.br = Ai::calc(&year, Type::Match(team_num));
                                    break year;
                                };
                            };
                            if check_cache(&year, team_num) {
                                send_and_check(year, team, ENV.firestore_collection.clone());
                            }
                            Ok(())
                        })?;
                }
                Ok(self)
            }
        }
    }
}

fn check_cache(year: &YearAround, team_num: &u16) -> bool {
    if let Ok(mut data) = CACHE_MATCH_AVG.lock() {
        if let Some(data) = data.get(team_num) {
            if data == year {
                return false;
            }
        }
        data.insert(team_num.to_owned(), year.to_owned());
    }
    true
}

fn send_and_check(year: YearAround, team: String, location: String) {
    thread::spawn(move || {
        if year.rp.avg.is_none() {
            warn!("Advanced data unavailable for team {}", &team)
        }
        if year.points.lowest == 10000 {
            warn!("Data unavailable for {} , skipping...", &team)
        } else {
            let e = YearStore::new(year).set_year(&team, &location);
            match e {
                Ok(e) => {
                    info!(
                        "Full data is found and is pushed to firstore for {}!\n\
                                \nWith Status: {}",
                        &team, e
                    );
                }
                Err(err) => {
                    error!(
                        "Failed for {}!\n\
                    with message {}",
                        &team, err
                    );
                }
            }
        }
    });
}

#[derive(Debug, Clone)]
pub enum SendType {
    Year(u16),
    Match,
}
