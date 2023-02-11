use crate::comp::parse::TeamYearAroundJsonParser;
use crate::ram::ENV;
use reqwest::Error;

pub async fn get_yearly(team: &str, year: u16) -> Result<TeamYearAroundJsonParser, Error> {
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/team/frc{team}/matches/{year}"
        ))
        .header("X-TBA-Auth-Key", &ENV.api_key)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
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
