use crate::ram::ENV;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
pub struct Http {
    pub team: u16,
    pub key: String,
}

impl Http {
    pub fn new(team: u16) -> Option<Http> {
        let response = reqwest::blocking::Client::new()
            .get("https://www.thebluealliance.com/api/v3/team/frc1403/events/2023/keys")
            .header("X-TBA-Auth-Key", &ENV.api_key)
            .send()
            .ok()?;
        let events = response.json::<Vec<String>>().ok()?;
        let data = events.last()?;
        Some(Http {
            team,
            key: data.to_owned(),
        })
    }
    pub fn get_data(&self) -> Option<()> {
        let response = reqwest::blocking::Client::new()
            .get(format!(
                "https://www.thebluealliance.com/api/v3/event/{}/oprs",
                self.key
            ))
            .header("X-TBA-Auth-Key", &ENV.api_key)
            .send()
            .ok()?;
        let events = response.json::<Opr>().ok()?;

        todo!()
        // https://www.thebluealliance.com/api/v3/event/2023njski/oprs
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Opr {
    pub ccwms: HashMap<String, f32>,
    pub dprs: HashMap<String, f32>,
    pub oprs: HashMap<String, f32>,
}
