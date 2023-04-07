pub mod get;
pub mod r#match;
use std::io::Error;
use std::process::Command;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::comp::avg::math::YearAround;

pub enum Version {
    Match,
    Year,
}

pub struct YearStore {
    pub(crate) year: YearAround,
}

impl YearStore {
    pub fn new(data: YearAround) -> Self {
        Self { year: data }
    }
    pub fn set_year(&self, team: &str, year_check: &str) -> Result<String, Error> {
        let data = SendYearAround {
            team: team.to_owned(),
            auto_high: self.year.auto.highest,
            auto_low: self.year.auto.lowest,
            auto_avg: self.year.auto.avg,
            pen_avg: self.year.pen.avg,
            pen_low: self.year.pen.lowest,
            pen_high: self.year.pen.highest,
            points_avg: self.year.points.avg,
            rp_high: self.year.rp.highest,
            rp_low: self.year.rp.lowest,
            rp_avg: self.year.rp.avg,
            points_low: self.year.points.lowest,
            points_high: self.year.points.highest,
            loss: self.year.losses,
            win_ratio: self.year.win_rato,
            score_graph: self.year.points.graph.clone(),
            penalty_graph: self.year.pen.graph.clone(),
            ranking_graph: self.year.rp.graph.clone(),
            win: self.year.wins,
            auto_graph: self.year.auto.graph.clone(),
            telop_game_pieces_highest: self.year.telop_game_pieces.highest,
            telop_game_points_highest: self.year.telop_game_points.highest,
            telop_game_pieces_lowest: self.year.telop_game_pieces.lowest,
            telop_game_points_lowest: self.year.telop_game_points.lowest,
            telop_game_pieces_avg: self.year.telop_game_pieces.avg,
            telop_game_points_avg: self.year.telop_game_points.avg,
            auto_game_pieces_highest: self.year.auto_game_pieces.highest,
            auto_game_points_highest: self.year.auto_game_points.highest,
            auto_game_pieces_lowest: self.year.auto_game_pieces.lowest,
            auto_game_points_lowest: self.year.auto_game_points.lowest,
            auto_game_pieces_avg: self.year.auto_game_pieces.avg,
            auto_game_points_avg: self.year.auto_game_points.avg,
            ekam_ai: self.year.ekam_ai,
            deviation: self.year.deviation,
        };
        let json = serde_json::to_string(&data)?;
        let result = Command::new("microService/firestore_send/bin")
            .arg(json)
            .arg(year_check)
            .arg(data.team)
            .output()?;
        let uft8_output = String::from_utf8(result.clone().stdout).unwrap_or(String::new());
        if uft8_output.is_empty() {
            return Ok(String::from_utf8(result.stderr).unwrap_or("Utf8 error".to_owned()));
        }
        Ok(uft8_output)
    }
}

//data to send
#[derive(Deserialize, Serialize)]
struct SendYearAround {
    team: String,
    #[serde(rename = "Auto Highest")]
    auto_high: Option<i16>,
    #[serde(rename = "Auto Lowest")]
    auto_low: Option<i16>,
    #[serde(rename = "Auto Avg")]
    auto_avg: Option<f32>,
    #[serde(rename = "Points Highest")]
    points_high: i16,
    #[serde(rename = "Points Lowest")]
    points_low: i16,
    #[serde(rename = "Points Avg")]
    points_avg: f32,
    #[serde(rename = "Ranking Point Highest")]
    rp_high: Option<i16>,
    #[serde(rename = "Ranking Point Lowest")]
    rp_low: Option<i16>,
    #[serde(rename = "Ranking Point Avg")]
    rp_avg: Option<f32>,
    #[serde(rename = "Penalty Highest")]
    pen_high: Option<i16>,
    #[serde(rename = "Penalty Lowest")]
    pen_low: Option<i16>,
    #[serde(rename = "Penalty Avg")]
    pen_avg: Option<f32>,
    #[serde(rename = "Wins")]
    win: i16,
    #[serde(rename = "Losses")]
    loss: i16,
    #[serde(rename = "Win loss ratio")]
    win_ratio: f32,
    #[serde(rename = "Standard Deviation")]
    deviation: f32,
    #[serde(rename = "Ekam Ai Rating")]
    ekam_ai: f32,
    score_graph: Vec<i16>,
    penalty_graph: Vec<i16>,
    ranking_graph: Vec<i16>,
    auto_graph: Vec<i16>,
    #[serde(rename = "Telop Game Pieces Highest")]
    telop_game_pieces_highest: Option<i16>,
    #[serde(rename = "Telop Game Pieces Points Highest")]
    telop_game_points_highest: Option<i16>,
    #[serde(rename = "Telop Game Pieces Lowest")]
    telop_game_pieces_lowest: Option<i16>,
    #[serde(rename = "Telop Game Pieces Points Lowest")]
    telop_game_points_lowest: Option<i16>,
    #[serde(rename = "Telop Game Pieces Avg")]
    telop_game_pieces_avg: Option<f32>,
    #[serde(rename = "Telop Game Pieces Points Avg")]
    telop_game_points_avg: Option<f32>,
    #[serde(rename = "Auto Game Pieces Highest")]
    auto_game_pieces_highest: Option<i16>,
    #[serde(rename = "Auto Game Pieces Points Highest")]
    auto_game_points_highest: Option<i16>,
    #[serde(rename = "Auto Game Pieces Pieces Lowest")]
    auto_game_pieces_lowest: Option<i16>,
    #[serde(rename = "Auto Game Pieces Points Lowest")]
    auto_game_points_lowest: Option<i16>,
    #[serde(rename = "Auto Game Pieces Avg")]
    auto_game_pieces_avg: Option<f32>,
    #[serde(rename = "Auto Game Pieces Points Avg")]
    auto_game_points_avg: Option<f32>,
}
