use crate::comp::parse::TeamYearAroundJsonParser;
use crate::ram::{ENV, YEAR};
use reqwest::Error;

use super::avg::year_around_main::SendType;
use crate::comp::shared::remove_frc;
use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type TeamKeys = Vec<Team>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Team {
    pub team_key: String,
}

pub async fn get_top_60() -> Vec<u16> {
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/district/{}fma/rankings",
            YEAR
        ))
        .header("X-TBA-Auth-Key", &ENV.api_key)
        .send()
        .await
        .unwrap();
    let keys = response.json::<TeamKeys>().await.unwrap();
    let (top, _) = keys.split_at(60);
    let str = remove_frc(top.iter().map(|key| key.team_key.to_owned()).collect());
    str.iter().filter_map(|str| str.parse().ok()).collect()
}

pub fn get_yearly(year: &SendType, team: &str) -> Result<TeamYearAroundJsonParser, Error> {
    let send_url: String = {
        match year {
            SendType::Year(year) => {
                format!("https://www.thebluealliance.com/api/v3/team/frc{team}/matches/{year}")
            }
            SendType::Match => {
                let update_where = &ENV.update_where;
                format!("https://www.thebluealliance.com/api/v3/event/{update_where}/matches")
            }
        }
    };
    let response = reqwest::blocking::Client::new()
        .get(send_url)
        .header("X-TBA-Auth-Key", &ENV.api_key)
        .send()?;
    response.json::<TeamYearAroundJsonParser>()
}

pub async fn get_match() -> reqwest::Result<TeamYearAroundJsonParser> {
    let update_where = &ENV.update_where;
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/event/{update_where}/matches"
        ))
        .header("X-TBA-Auth-Key", &ENV.api_key)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
}
