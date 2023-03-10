use crate::comp::parse::TeamYearAroundJsonParser;
use crate::ram::ENV;
use reqwest::Error;

use super::avg::year_around_main::SendType;

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
    let cool = response.json::<TeamYearAroundJsonParser>();
    cool
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
