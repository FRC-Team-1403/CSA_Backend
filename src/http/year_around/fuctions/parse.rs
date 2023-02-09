//this is the team year around avg, to change please go to https://transform.tools/json-to-rust-serde
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub type TeamYearAroundJsonParser = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root2 {
    pub alliances: Alliances,
    #[serde(rename = "comp_level")]
    pub comp_level: String,
    #[serde(rename = "event_key")]
    pub event_key: String,
    pub key: String,
    #[serde(rename = "match_number")]
    pub match_number: i8,
    #[serde(rename = "score_breakdown")]
    pub score_breakdown: Option<ScoreBreakdown>,
    // pub videos: Vec<Video>,
    #[serde(rename = "winning_alliance")]
    pub winning_alliance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alliances {
    pub blue: Blue,
    pub red: Red,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blue {
    #[serde(rename = "dq_team_keys")]
    pub dq_team_keys: Vec<Value>,
    pub score: i16,
    #[serde(rename = "surrogate_team_keys")]
    pub surrogate_team_keys: Vec<Value>,
    #[serde(rename = "team_keys")]
    pub team_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Red {
    #[serde(rename = "dq_team_keys")]
    pub dq_team_keys: Vec<Value>,
    pub score: i16,
    #[serde(rename = "surrogate_team_keys")]
    pub surrogate_team_keys: Vec<Value>,
    #[serde(rename = "team_keys")]
    pub team_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreBreakdown {
    pub blue: Blue2,
    pub red: Red2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blue2 {
    pub rp: i16,
    pub auto_points: i16,
    pub foul_count: i16,
    pub foul_points: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Red2 {
    pub rp: i16,
    pub auto_points: i16,
    pub foul_count: i16,
    pub foul_points: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub key: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
