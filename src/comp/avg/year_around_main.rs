#![allow(clippy::needless_late_init)]

use crate::comp::ai::Ai;
use crate::comp::avg::math::YearAround;
use crate::comp::http::get_yearly;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::db::firebase::YearStore;
use crate::db::redis_functions::RedisDb;
use crate::ram::{CACHE_MATCH_AVG, CACHE_YEAR_AVG, ENV};
use futures::executor::block_on;
use log::{error, info, warn};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

const PUBLIC_CACHE: u16 = 16969;

#[derive(Clone, Debug)]
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
    fn check_cache(&mut self, json: TeamYearAroundJsonParser, what: &SendType, team: &u16) -> bool {
        let loc: u16;
        match what {
            SendType::Year(_) => {
                if let Some(compare) = self.cache.get(team) {
                    if compare == &json {
                        info!("Skipping {team}, The data is updated");
                        return false;
                    }
                }
                loc = *team;
            }
            SendType::Match => {
                if let Some(compare) = self.cache.get(&PUBLIC_CACHE) {
                    if compare == &json {
                        info!("Skipping match update,The data is updated");
                        return false;
                    }
                }
                loc = PUBLIC_CACHE;
            }
        }
        self.updated = true;
        self.cache.insert(loc, json);
        true
    }
    pub fn get_new_data(what: SendType, frc: &str) -> Option<TeamYearAroundJsonParser> {
        let mut _failed: u8 = 0;
        loop {
            let response = get_yearly(&what, frc);
            match response {
                Ok(json) => {
                    return Some(json);
                }
                Err(err) => {
                    error!("ERROR: {}", err);
                    if _failed == 120 {
                        error!("failed to get data due to: {}", err);
                        return None;
                    }
                    _failed += 1
                }
            }
        }
    }
    pub fn update(mut self, what: SendType) -> Result<Self, Self> {
        match what {
            SendType::Year(_) => {
                let cache = tokio::sync::Mutex::new(self.clone());
                ENV.teams.par_iter().try_for_each(|team_num| {
                    info!("Doing Year Around For Team {team_num}");
                    let team = team_num.to_string();
                    let Some(json) =
                        Self::get_new_data(what.clone(), &team) else {
                        return Err(self.clone());
                    };
                    let mut _allow: bool =
                        block_on(cache.lock()).check_cache(json.clone(), &what, team_num);
                    if _allow {
                        let year = YearAround::new(json).calculate(&team);
                        let Ok(mut year) = year else {
                            error!("failed to parse data");
                            return Err(self.clone());
                        };
                        year.ekam_ai = 0.0;
                        thread::spawn(move || {
                            CACHE_YEAR_AVG
                                .lock()
                                .expect("Dead Lock")
                                .insert(team_num.to_owned(), year);
                            info!("Year Data Set For Team: {team_num}")
                        });
                        // send_and_check(year, team, year_check.to_string());
                    }
                    Ok(())
                })?;
                let data = block_on(cache.lock());
                self.cache = data.cache.clone();
                self.updated = data.updated;
                Ok(self)
            }
            SendType::Match => {
                let Some(json) =
                    Self::get_new_data(what.clone(), "69") else {
                    return Err(self);
                };
                let mut _allow = self.check_cache(json.clone(), &what, &69);
                if _allow {
                    let redis_db = Mutex::new(RedisDb::new());
                    let calc = YearAround::new(json);
                    ENV.teams
                        .par_iter()
                        .try_for_each(|team_num| -> Result<(), Self> {
                            let team_calc = calc.clone();
                            let team = team_num.to_string();
                            let year = team_calc.calculate(&team);
                            let year = loop {
                                if let Ok(mut year) = year {
                                    year.ekam_ai = Ai::calc_match(&year, team_num);
                                    year.contributed_score =
                                        Ai::predict_match_score(&year, team_num);
                                    break year;
                                };
                            };
                            if check_cache(&year, team_num) {
                                send_and_check(
                                    year.clone(),
                                    team,
                                    ENV.firestore_collection.clone(),
                                );
                                send_redis(team_num, &year, &redis_db);
                            }
                            Ok(())
                        })?;
                }
                Ok(self)
            }
        }
    }
}

fn send_redis(team: &u16, data: &YearAround, what: &Mutex<Option<RedisDb>>) {
    loop {
        if let Ok(mut cool_data) = what.lock() {
            if cool_data.is_some() {
                cool_data.as_mut().unwrap().send_avg_redis(team, data);
            }
            return;
        } else {
            panic!("Dead Lock With Send Redis Db");
        }
    }
}

fn check_cache(year: &YearAround, team_num: &u16) -> bool {
    if let Some(data) = CACHE_MATCH_AVG.lock().expect("Dead Lock").get(team_num) {
        if data == year {
            return false;
        }
    }
    CACHE_MATCH_AVG
        .lock()
        .expect("Dead Lock")
        .insert(team_num.to_owned(), year.to_owned());
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
