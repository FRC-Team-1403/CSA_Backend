use crate::comp::event::Event;
use crate::comp::parse::Root2;
use crate::comp::shared::{get_breakdown_data, get_score, get_teammates, Team};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    #[serde(rename = "team")]
    pub team: u16,
    #[serde(rename = "Penalty Points")]
    pub penalty: Option<i16>,
    #[serde(rename = "Auto Points")]
    pub auto: Option<i16>,
    #[serde(rename = "Ranking Points")]
    pub rp: Option<i16>,
    #[serde(rename = "Match Number")]
    pub match_number: u8,
    #[serde(rename = "Team Members")]
    pub team_members: Vec<String>,
    #[serde(rename = "Total Points")]
    pub score: i16,
    #[serde(rename = "Video")]
    pub video: Option<String>,
    #[serde(rename = "Auto Charge Station Level")]
    pub auto_level: Option<String>,
    #[serde(rename = "End Game Charge Station Level")]
    pub end_level: Option<String>,
    #[serde(rename = "Sustainability Bonus Achieved")]
    pub sustain_bonus: Option<bool>,
}

impl Event {
    pub fn math(&self, check_team: u16) -> Result<Vec<EventData>, Error> {
        let mut return_data = vec![];
        let matches = self.find_team(check_team);
        for (game_json, team) in matches {
            let (auto, penalty, rp, auto_l, end_l, bonus) =
                get_breakdown_data(game_json.score_breakdown.clone(), &team);
            let score = get_score(&team, game_json.clone());
            if score != -1 {
                return_data.push(EventData {
                    team: check_team,
                    video: Self::get_video(&game_json),
                    auto_level: auto_l,
                    rp,
                    penalty,
                    score,
                    match_number: game_json.match_number,
                    team_members: get_teammates(team, game_json),
                    auto,
                    end_level: end_l,
                    sustain_bonus: bonus,
                });
            }
        }
        Ok(return_data)
    }
    fn find_team(&self, check_team: u16) -> Vec<(Root2, Team)> {
        let mut return_data = vec![];
        let team = format!("frc{check_team}");
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
