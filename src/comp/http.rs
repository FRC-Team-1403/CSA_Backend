use crate::comp::parse::TeamYearAroundJsonParser;
use crate::config::UPDATE_WHERE;
use crate::constant::API_KEY;
use reqwest::Error;

pub async fn get_yearly(team: &str, year: u16) -> Result<TeamYearAroundJsonParser, Error> {
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/team/frc{team}/matches/{year}"
        ))
        .header("X-TBA-Auth-Key", API_KEY)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
}

pub async fn get_match() -> reqwest::Result<TeamYearAroundJsonParser> {
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/event/{UPDATE_WHERE}/matches"
        ))
        .header("X-TBA-Auth-Key", API_KEY)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
}
