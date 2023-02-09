use crate::http::shared::{Shared, Team};
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;
use std::fmt::Error;

pub struct Data {
    pub avg: f32,
    pub lowest: i16,
    pub highest: i16,
}

pub struct OData {
    pub avg: Option<f32>,
    pub lowest: Option<i16>,
    pub highest: Option<i16>,
}

impl OData {
    fn new() -> Self {
        Self {
            avg: None,
            lowest: None,
            highest: None,
        }
    }
}

impl Data {
    fn new() -> Self {
        Self {
            avg: 0.0,
            lowest: 10000,
            highest: 0,
        }
    }
}

pub struct YearAround {
    pub data: Option<TeamYearAroundJsonParser>,
    pub points: Data,
    pub wins: i16,
    pub losses: i16,
    pub win_rato: f32,
    pub matches_played: i16,
    pub pen: OData,
    pub rp: OData,
    pub auto: OData,
}

impl Shared for YearAround {}
impl YearAround {
    pub fn new(data: TeamYearAroundJsonParser) -> Self {
        Self {
            data: Some(data),
            points: Data::new(),
            wins: 0,
            losses: 0,
            win_rato: 0.0,
            matches_played: 0,
            pen: OData::new(),
            rp: OData::new(),
            auto: OData::new(),
        }
    }
    //noinspection ALL
    pub fn calculate(mut self, team: &str) -> Result<Self, std::fmt::Error> {
        let mut avg_score = vec![];
        let mut avg_rp = vec![];
        let mut avg_foul = vec![];
        let mut avg_auto = vec![];
        //looping through the JSON
        for json in self.data.clone().ok_or(Error)? {
            if json.alliances.red.team_keys.contains(&format!("frc{team}")) {
                (self.losses, self.wins) = Self::check_win(
                    Team::Red,
                    self.losses,
                    self.wins,
                    json.winning_alliance.trim(),
                );
                let (auto_points, foul, rp) =
                    Self::get_breakdown_data(json.score_breakdown, &Team::Red);
                let handle = HandleData {
                    rp,
                    score: json.alliances.red.score,
                    avg_score,
                    avg_rp,
                    avg_foul,
                    avg_auto,
                    auto_points,
                    foul,
                };
                let new_data: HandleData;
                (self, new_data) = self.handle(handle);
                avg_score = new_data.avg_score;
                avg_foul = new_data.avg_foul;
                avg_rp = new_data.avg_rp;
                avg_auto = new_data.avg_auto;
            } else {
                (self.losses, self.wins) = Self::check_win(
                    Team::Blue,
                    self.losses,
                    self.wins,
                    json.winning_alliance.trim(),
                );
                let (auto_points, foul, rp) =
                    Self::get_breakdown_data(json.score_breakdown, &Team::Blue);
                let mut handle = HandleData {
                    rp,
                    score: json.alliances.red.score,
                    avg_score,
                    avg_rp,
                    avg_foul,
                    avg_auto,
                    auto_points,
                    foul,
                };
                (self, handle) = self.handle(handle);
                avg_score = handle.avg_score;
                avg_foul = handle.avg_foul;
                avg_rp = handle.avg_rp;
                avg_auto = handle.avg_auto;
            }
        }
        self.data = None;
        self.rp.avg = Some(self.avg(avg_rp));
        self.pen.avg = Some(self.avg(avg_foul));
        self.points.avg = self.avg(avg_score);
        self.auto.avg = Some(self.avg(avg_auto));
        self.win_rato = self.wins as f32 / self.losses as f32;
        self.matches_played = self.wins + self.losses;

        Ok(self)
    }
    fn handle(mut self, mut return_data: HandleData) -> (Self, HandleData) {
        //happens if match breakdown works
        if let Some(rp) = return_data.rp {
            let foul = return_data.foul.unwrap_or(0);
            let auto_points = return_data.auto_points.unwrap_or(0);
            //lowest code
            self.rp.highest = Some(Self::compare_highest(self.rp.highest.unwrap_or(0), rp));
            self.pen.highest = Some(Self::compare_highest(self.pen.highest.unwrap_or(0), foul));
            self.auto.highest = Some(Self::compare_highest(
                self.auto.highest.unwrap_or(0),
                auto_points,
            ));
            //lowest code
            self.rp.lowest = Some(Self::compare_lowest(self.rp.lowest.unwrap_or(1000), rp));
            self.pen.lowest = Some(Self::compare_lowest(self.pen.lowest.unwrap_or(1000), foul));
            self.auto.lowest = Some(Self::compare_lowest(
                self.auto.lowest.unwrap_or(1000),
                auto_points,
            ));
            //avg code
            return_data.avg_foul.push(foul);
            return_data.avg_auto.push(auto_points);
            return_data.avg_rp.push(rp);
            //avg
        }
        self.points.highest = Self::compare_highest(self.points.highest, return_data.score);
        //lowest code
        self.points.lowest = Self::compare_lowest(self.points.lowest, return_data.score);
        //adding of avg
        return_data.avg_score.push(return_data.score);
        (self, return_data)
    }
}

struct HandleData {
    score: i16,
    avg_score: Vec<i16>,
    avg_rp: Vec<i16>,
    avg_foul: Vec<i16>,
    avg_auto: Vec<i16>,
    rp: Option<i16>,
    auto_points: Option<i16>,
    foul: Option<i16>,
}
