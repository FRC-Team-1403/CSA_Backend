use std::collections::HashMap;

use reqwest::header;
use reqwest::Client;

#[derive(Debug, serde::Deserialize)]
struct Team {
    team_number: String,
    state_prov: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct Event {
    name: Option<String>,
}

async fn get_teams(
    state_key: &str,
    api_key: &str,
    max_page: i32,
    url: &str,
) -> Result<Vec<String>, reqwest::Error> {
    let mut team_keys = Vec::new();
    let client = Client::new();
    for page in 0..max_page {
        let response = client
            .get(&format!(url, page))
            .header(header::USER_AGENT, "reqwest")
            .header("X-TBA-Auth-Key", api_key)
            .send()
            .await?;
        if response.status().is_success() {
            let teams: Vec<Team> = response.json().await?;
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

async fn get_team_competing(
    event_key: &str,
    api_key: &str,
    max_page: i32,
    url: &str,
) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get(&format!(url, event_key))
        .header(header::USER_AGENT, "reqwest")
        .header("X-TBA-Auth-Key", api_key)
        .send()
        .await?;
    let mut teams = Vec::new();
    if response.status().is_success() {
        let teams: Vec<HashMap<String, String>> = response.json().await?;
        for team in teams {
            if let Some(team_number) = team.get("team_number") {
                teams.push(team_number.to_owned());
            }
        }
        Ok(teams.join(", "))
    } else {
        Ok(String::new())
    }
}

async fn get_event_name(
    event_key: &str,
    api_key: &str,
    url: &str,
) -> Result<Option<String>, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get(&format!(url, event_key))
        .header(header::USER_AGENT, "reqwest")
        .header("X-TBA-Auth-Key", api_key)
        .send()
        .await?;
    if response.status().is_success() {
        let event: Event = response.json().await?;
        Ok(event.name)
    } else {
        Ok(None)
    }
}

/*
Example Usage
#[tokio::main]
async fn main() {
    let api_key = "API_KEY";
    let state_key = "State";
    let max_page = 20;
    let url = "https://www.thebluealliance.com/api/v3/teams/{}";
    let teams = get_teams(state_key, api_key, max_page, url).await.unwrap();
    println!("{:?}", teams);

    let event_key = "EVENT_KEY";
    let url = "https://www.thebluealliance.com/api/v3/event/{}/teams";
    let teams = get_team_competing(event_key, api_key, max_page, url).await.unwrap();
    println!("{}", teams);

    let event_key
