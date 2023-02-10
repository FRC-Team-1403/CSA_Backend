use crate::http::event::Event;
use crate::http::shared::{Shared, Team};
use crate::http::year_around::fuctions::parse::Root2;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    #[serde(rename = "team")]
    team: u16,
    #[serde(rename = "Penalty Points")]
    penalty: Option<i16>,
    #[serde(rename = "Auto Points")]
    auto: Option<i16>,
    #[serde(rename = "Ranking Points")]
    rp: Option<i16>,
    #[serde(rename = "Match Number")]
    match_number: i8,
    #[serde(rename = "Team Members")]
    team_members: Vec<String>,
    #[serde(rename = "Score")]
    score: i16,
    #[serde(rename = "Video")]
    video: Option<String>,
}
//https://www.youtube.com/watch?v=_G38qoLkH4A

impl Shared for Event {}

impl Event {
    pub fn math(&self) -> Result<Vec<EventData>, Error> {
        let mut return_data = vec![];
        let matches = self.find_team();
        for (game_json, team) in matches {
            let (auto, penalty, rp) =
                Self::get_breakdown_data(game_json.score_breakdown.clone(), &team);
            return_data.push(EventData {
                team: self.team,
                video: Self::get_video(&game_json),
                rp,
                penalty,
                score: Self::get_score(&team, game_json.clone()),
                match_number: game_json.match_number,
                team_members: Self::get_teammates(team, game_json),
                auto,
            });
        }
        Ok(return_data)
    }
    fn find_team(&self) -> Vec<(Root2, Team)> {
        let mut return_data = vec![];
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
    fn get_video(game_json: &Root2) -> Option<String> {
        let Some(video) = game_json.videos.get(0) else {
            return None
        };
        Some(format!("https://www.youtube.com/watch?v={}", video.key))
    }
}
