use crate::comp::event::math::EventData;
use crate::ram::{CACHE_MATCH, ENV};
use log::warn;
use rayon::prelude::*;
use std::process::Command;
use std::{io, thread};

pub struct MatchStore {
    data: Vec<EventData>,
}

impl MatchStore {
    pub fn new(data: Vec<EventData>) -> Self {
        Self { data }
    }
    pub fn send(self) {
        thread::spawn(move || {
            let _ = self
                .data
                .par_iter()
                .try_for_each(|raw_json| -> Result<(), io::Error> {
                    if check_cache(raw_json, &raw_json.team) {
                        let json = serde_json::to_string(&raw_json)?;
                        let firestore_location = &ENV.firestore_collection;
                        let result = Command::new("microService/firestore_send/bin")
                            .args([
                                json,
                                firestore_location.to_owned(),
                                raw_json.team.to_string(),
                                "Matches".to_owned(),
                                raw_json.match_number.to_string(),
                            ])
                            .output()?;
                        let uft8_output =
                            String::from_utf8(result.clone().stdout).unwrap_or(String::new());
                        if uft8_output.is_empty() {
                            warn!(
                                "{}",
                                String::from_utf8(result.stderr).unwrap_or("Utf8 error".to_owned())
                            );
                        }
                        println!("Sent for {} with output of {uft8_output}", raw_json.team);
                    }
                    Ok(())
                });
        });
    }
}
fn check_cache(year: &EventData, team_num: &u16) -> bool {
    if let Ok(mut hash) = CACHE_MATCH.lock() {
        let add;
        if let Some(data) = hash.get(team_num) {
            for check in data {
                if check == year {
                    return false;
                }
            }
            let mut data = data.to_owned();
            data.push(year.to_owned());
            add = data
        } else {
            add = vec![year.to_owned()]
        }
        hash.insert(team_num.to_owned(), add);
    }
    true
}
