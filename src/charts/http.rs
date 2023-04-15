use crate::ram::ENV;
use crate::startup::tba::Tba;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::thread;

pub struct Http {
    pub team: u16,
    pub key: String,
}

impl Http {
    pub fn new(team: u16) -> Option<Http> {
        let response = reqwest::blocking::Client::new()
            .get(format!(
                "https://www.thebluealliance.com/api/v3/team/frc{}/events/2023/keys",
                team
            ))
            .header("X-TBA-Auth-Key", &ENV.api_key)
            .send()
            .ok()?;
        let mut events = response.json::<Vec<String>>().ok()?;
        events.retain(|event| event != &ENV.code);
        let data = events.first()?;
        let check_data = data.to_owned();
        let check_events = events.clone();
        thread::spawn(move || {
            let check_data = check_data;
            log::info!(
                "event: {:?} for team {team} with events of {:?}, with key of {}\n",
                Tba::get_event(&check_data, &ENV.api_key).unwrap(),
                &check_events,
                check_data
            );
        });
        Some(Http {
            team,
            key: data.to_owned(),
        })
    }
    pub fn get_data(&self) -> Option<Team> {
        let response = reqwest::blocking::Client::new()
            .get(format!(
                "https://www.thebluealliance.com/api/v3/event/{}/oprs",
                self.key
            ))
            .header("X-TBA-Auth-Key", &ENV.api_key)
            .send()
            .ok()?;
        let events = response.json::<OprParser>().ok()?;
        let team = format!("frc{}", self.team);
        let ccwms = events.ccwms.get(&team)?.to_owned();
        let dprs = events.dprs.get(&team)?.to_owned();
        let oprs = events.oprs.get(&team)?.to_owned();
        Some(Team { ccwms, dprs, oprs })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OprParser {
    pub ccwms: HashMap<String, f32>,
    pub dprs: HashMap<String, f32>,
    pub oprs: HashMap<String, f32>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Team {
    pub ccwms: f32,
    pub dprs: f32,
    pub oprs: f32,
}
