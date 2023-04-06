use crate::comp::parse::TeamYearAroundJsonParser;
use crate::comp::shared::{
    avg, check_win, compare_highest, compare_lowest, deviation, get_breakdown_data, Team,
};
use std::fmt::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub avg: f32,
    pub lowest: i16,
    pub highest: i16,
    pub graph: Vec<i16>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OData {
    pub avg: Option<f32>,
    pub lowest: Option<i16>,
    pub highest: Option<i16>,
    pub graph: Vec<i16>,
}

impl OData {
    fn new() -> Self {
        Self {
            graph: vec![],
            avg: None,
            lowest: None,
            highest: None,
        }
    }
}
impl Data {
    fn new() -> Self {
        Self {
            graph: vec![],
            avg: 0.0,
            lowest: 10000,
            highest: 0,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
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
    pub deviation: f32,
    pub ekam_ai: f32,
}

impl YearAround {
    pub fn new(data: TeamYearAroundJsonParser) -> Self {
        Self {
            deviation: 0.0,
            ekam_ai: 0.0,
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
            if json.alliances.red.team_keys.contains(&format!("frc{team}"))
                && json.alliances.red.score != -1
            {
                (self.losses, self.wins) = check_win(
                    Team::Red,
                    self.losses,
                    self.wins,
                    json.winning_alliance.trim(),
                );
                let breakdown = get_breakdown_data(json.score_breakdown, &Team::Red);
                let handle = HandleData {
                    rp: breakdown.rp,
                    score: json.alliances.red.score,
                    avg_score,
                    avg_rp,
                    avg_foul,
                    avg_auto,
                    auto_points: breakdown.auto_points,
                    foul: breakdown.foul_points,
                };
                let new_data: HandleData;
                (self, new_data) = self.handle(handle);
                avg_score = new_data.avg_score;
                avg_foul = new_data.avg_foul;
                avg_rp = new_data.avg_rp;
                avg_auto = new_data.avg_auto;
            } else if json
                .alliances
                .blue
                .team_keys
                .contains(&format!("frc{team}"))
                && json.alliances.blue.score != -1
            {
                (self.losses, self.wins) = check_win(
                    Team::Blue,
                    self.losses,
                    self.wins,
                    json.winning_alliance.trim(),
                );
                let breakdown = get_breakdown_data(json.score_breakdown, &Team::Blue);
                let mut handle = HandleData {
                    rp: breakdown.rp,
                    score: json.alliances.red.score,
                    avg_score,
                    avg_rp,
                    avg_foul,
                    avg_auto,
                    auto_points: breakdown.auto_points,
                    foul: breakdown.foul_points,
                };
                (self, handle) = self.handle(handle);
                avg_score = handle.avg_score;
                avg_foul = handle.avg_foul;
                avg_rp = handle.avg_rp;
                avg_auto = handle.avg_auto;
            }
        }
        self.data = None;
        self.deviation = deviation(&avg_score);
        self.rp.graph = avg_rp.clone();
        self.rp.avg = Some(avg(avg_rp));
        self.pen.graph = avg_foul.clone();
        self.pen.avg = Some(avg(avg_foul));
        self.points.graph = avg_score.clone();
        self.points.avg = avg(avg_score);
        self.auto.graph = avg_auto.clone();
        self.auto.avg = Some(avg(avg_auto));
        self.matches_played = self.wins + self.losses;
        // This MUST BE CALLED LAST
        self.calc_ratio();
        Ok(self)
    }
    fn calc_ratio(&mut self) {
        if self.matches_played == 0 {
            self.win_rato = 0.0;
            return;
        }
        self.win_rato = self.wins as f32 / self.matches_played as f32;
    }

    fn handle(mut self, mut return_data: HandleData) -> (Self, HandleData) {
        //happens if match breakdown works
        if let Some(rp) = return_data.rp {
            let foul = return_data.foul.unwrap_or(0);
            let auto_points = return_data.auto_points.unwrap_or(0);
            //lowest code
            self.rp.highest = Some(compare_highest(self.rp.highest.unwrap_or(0), rp));
            self.pen.highest = Some(compare_highest(self.pen.highest.unwrap_or(0), foul));
            self.auto.highest = Some(compare_highest(self.auto.highest.unwrap_or(0), auto_points));
            //lowest code
            self.rp.lowest = Some(compare_lowest(self.rp.lowest.unwrap_or(1000), rp));
            self.pen.lowest = Some(compare_lowest(self.pen.lowest.unwrap_or(1000), foul));
            self.auto.lowest = Some(compare_lowest(
                self.auto.lowest.unwrap_or(1000),
                auto_points,
            ));
            //avg code
            return_data.avg_foul.push(foul);
            return_data.avg_auto.push(auto_points);
            return_data.avg_rp.push(rp);
            //avg
        }
        self.points.highest = compare_highest(self.points.highest, return_data.score);
        //lowest code
        self.points.lowest = compare_lowest(self.points.lowest, return_data.score);
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
