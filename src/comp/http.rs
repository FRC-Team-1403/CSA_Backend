use openssl::envelope::Open;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::ram::ENV;
use reqwest::Error;
use pyo3::prelude::*;
use pyo3::types::PyFunction;

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


pub fn get_all_teams(event_key: &str, api_key: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.get("version")?.extract()?;

        let my_module = py.import("/microService/get_all_teams/get_all_teams")?;
        let my_func: &PyFunction = my_module.get("get_teams")?.extract()?;
        let result = my_func.call((event_key, api_key), None)?;
        return result;
    })
}

pub fn get_competition_name(event_key: &str, api_key: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.get("version")?.extract()?;

        let my_module = py.import("/microService/get_event_name/get_event_name")?;
        let my_func: &PyFunction = my_module.get("get_event_name")?.extract()?;
        let result = my_func.call((event_key, api_key), None)?;
        return result;
    })
}

pub fn get_competing_teams(event_key: &str, api_key: &str) -> PyResult<String> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.get("version")?.extract()?;

        let my_module = py.import("/microService/get_all_teams/get_all_teams")?;
        let my_func: &PyFunction = my_module.get("get_team_competing")?.extract()?;
        let result = my_func.call((event_key, api_key), None)?;

        return result;
    })
}