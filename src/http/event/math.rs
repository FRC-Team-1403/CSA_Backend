use crate::http::event::Event;
use crate::http::shared::{Shared, Team};
use crate::http::year_around::fuctions::parse::Root2;
use crate::http::year_around::math::YearAround;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    #[serde(rename = "Match Number")]
    match_number: i8,
    #[serde(rename = "Team Members")]
    team_members: Vec<String>,
}

impl Shared for Event {}

impl Event {
    pub fn math(&self) -> Result<Vec<EventData>, Error> {
        let mut return_data = vec![];
        let matches = self.find_team();
        for (game_json, team) in matches {
            return_data.push(EventData {
                match_number: game_json.match_number,
                team_members: Self::get_teammates(team, game_json),
            });
        }
        Ok(return_data)
    }
    fn find_team(&self) -> Vec<(Root2, Team)> {
        let mut return_data = vec![];
        let mut return_team: Vec<Team> = vec![];
        let team = format!("frc{}", self.team);
        for data in self.new_data.clone() {
            if data.alliances.red.team_keys.contains(&team) {
                return_data.push((data, Team::Red));
            } else if data.alliances.blue.team_keys.contains(&team) {
                return_data.push((data, Team::Blue));
            }
        }
        return_data
    }
}
