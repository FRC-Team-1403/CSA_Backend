use reqwest::header::HeaderMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Team {
    team_number: String,
    state_prov: Option<String>,
}

fn get_teams(
    state_key: &str,
    api_key: &str,
    max_page: i32,
    url: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-TBA-Auth-Key",
        api_key.parse().expect("Failed to parse api key header value"),
    );
    let mut team_keys = Vec::new();
    for page in 0..max_page {
        let page_url = format!("{}{}", url, page);
        let response = client.get(&page_url).headers(headers.clone()).send()?;
        if response.status().is_success() {
            let teams: Vec<Team> = response.json()?;
            for team in teams {
                if state_key != "*" {
                    if let Some(state_prov) = team.state_prov {
                        if state_prov == state_key {
                            team_keys.push(team.team_number);
                        }
                    }
                } else {
                    team_keys.push(team.team_number);
                }
            }
        }
    }
    Ok(team_keys)
}

fn get_team_competing(
    event_key: &str,
    api_key: &str,
    max_page: i32,
    url: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-TBA-Auth-Key",
        api_key.parse().expect("Failed to parse api key header value"),
    );
    let mut teams = Vec::new();
    let response = client.get(&format!(url, event_key)).headers(headers).send()?;
    if response.status().is_success() {
        let team_list: Vec<HashMap<String, String>> = response.json()?;
        for team in team_list {
            if let Some(team_number) = team.get("team_number") {
                teams.push(team_number.to_string());
            }
        }
        Ok(teams.join(", "))
    } else {
        Ok("".to_string())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let x = get_teams("State", "API_KEY", 20, "https://www.thebluealliance.com/api/v3/teams/")?;
    println!("{:?}", x);
    Ok(())
}
