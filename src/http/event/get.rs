use crate::config::UPDATE_WHERE;
use crate::constant::API_KEY;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;

pub async fn get() -> reqwest::Result<TeamYearAroundJsonParser> {
    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/event/{UPDATE_WHERE}/matches"
        ))
        .header("X-TBA-Auth-Key", API_KEY)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
}