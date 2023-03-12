use rayon::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventJson {
    pub name: String,
}

pub type JSON = Vec<String>;

pub struct Tba {}

impl Tba {
    pub fn new() -> Tba {
        Self {}
    }
    pub fn get_event(key: &str, api_key: &str) -> reqwest::Result<String> {
        let response = reqwest::blocking::Client::new()
            .get(format!(
                "https://www.thebluealliance.com/api/v3/event/{key}"
            ))
            .header("X-TBA-Auth-Key", api_key)
            .send()?;
        let json: EventJson = response.json()?;
        Ok(json.name)
    }
    pub fn get_teams(key: &str, api_key: &str) -> reqwest::Result<Vec<u16>> {
        let response = reqwest::blocking::Client::new()
            .get(format!(
                "https://www.thebluealliance.com/api/v3/event/{key}/teams/keys"
            ))
            .header("X-TBA-Auth-Key", api_key)
            .send()?;
        let json: JSON = response.json()?;
        let teams: Vec<u16> = json
            .par_iter()
            .map(|team| team.replace("frc", "").parse().unwrap_or(0))
            .collect();
        Ok(teams)
    }
}
