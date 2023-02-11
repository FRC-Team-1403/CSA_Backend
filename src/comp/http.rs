use crate::comp::parse::TeamYearAroundJsonParser;
use dotenv;
use std::env;
use std::path::{Path};
use reqwest::Error;

pub async fn get_yearly(team: &str, year: u16) -> Result<TeamYearAroundJsonParser, Error> {
    let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
    dotenv::from_path(my_path.as_path());
    let api_key = dotenv::var("API_KEY").unwrap();

    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/team/frc{team}/matches/{year}"
        ))
        .header("X-TBA-Auth-Key", api_key)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
}

pub async fn get_match() -> reqwest::Result<TeamYearAroundJsonParser> {
    let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
    dotenv::from_path(my_path.as_path());
    let api_key = dotenv::var("API_KEY").unwrap();
    let update_where = dotenv::var("UPDATE_WHERE").unwrap();

    let response = reqwest::Client::new()
        .get(format!(
            "https://www.thebluealliance.com/api/v3/event/{update_where}/matches"
        ))
        .header("X-TBA-Auth-Key", api_key)
        .send()
        .await?;
    response.json::<TeamYearAroundJsonParser>().await
}
