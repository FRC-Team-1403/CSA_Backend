use crate::constant::API_KEY;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;
use reqwest::Error;
pub mod parse;

pub async fn get(team: &str, year: u16) -> Result<TeamYearAroundJsonParser, Error> {
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/team/frc{team}/matches/{year}"
        ))
        .header("X-TBA-Auth-Key", API_KEY)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
}
