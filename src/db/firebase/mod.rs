pub mod r#match;

use std::io::Error;
use std::process::Command;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::comp::avg::math::YearAround;

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
    score_graph: Vec<i16>,
    penalty_graph: Vec<i16>,
    ranking_graph: Vec<i16>,
    auto_graph: Vec<i16>,
}
