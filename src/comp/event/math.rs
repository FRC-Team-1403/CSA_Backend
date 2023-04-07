use crate::comp::event::Event;
use crate::comp::parse::Root2;
use crate::comp::shared::{get_breakdown_data, get_score, get_teammates, Team};
use rayon::prelude::*;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    #[serde(rename = "team")]
    pub team: u16,
    #[serde(rename = "Auto Game Pieces")]
    pub auto_game_pieces: Option<i16>,
    #[serde(rename = "Auto Game Points")]
    pub auto_game_points: Option<i16>,
    #[serde(rename = "Teleop Game Pieces")]
    pub teleop_game_pieces: Option<i16>,
    #[serde(rename = "Teleop Game Points")]
    pub teleop_game_points: Option<i16>,
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
    pub fn math(&self, check_team: u16) -> Vec<EventData> {
        self.find_team(check_team)
            .par_iter()
            .filter_map(|(game_json, team)| {
                let breakdown = get_breakdown_data(game_json.score_breakdown.clone(), team);
                let score = get_score(team, game_json.clone());
                if score != -1 {
                    Some(EventData {
                        team: check_team,
                        auto_game_pieces: breakdown.auto_game_piece_count,
                        auto_game_points: breakdown.auto_game_piece_points,
                        teleop_game_pieces: breakdown.telop_game_piece_count,
                        video: Self::get_video(game_json),
                        auto_level: breakdown.auto_auto_bridge_state,
                        rp: breakdown.rp,
                        penalty: breakdown.foul_points,
                        score,
                        match_number: game_json.match_number,
                        team_members: get_teammates(team.to_owned(), game_json.to_owned()),
                        auto: breakdown.auto_points,
                        end_level: breakdown.end_game_bridge_state,
                        sustain_bonus: breakdown.sustainability_bonus_achieved,
                        teleop_game_points: breakdown.telop_game_piece_points,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    fn find_team(&self, check_team: u16) -> Vec<(Root2, Team)> {
        let team = format!("frc{check_team}");
        self.new_data
            .par_iter()
            .filter_map(|data| {
                if data.alliances.red.team_keys.contains(&team) {
                    Some((data.to_owned(), Team::Red))
                } else if data.alliances.blue.team_keys.contains(&team) {
                    Some((data.to_owned(), Team::Blue))
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_video(game_json: &Root2) -> Option<String> {
        let Some(video) = game_json.videos.get(0) else {
            return None;
        };
        Some(format!("https://www.youtube.com/watch?v={}", video.key))
    }
}
