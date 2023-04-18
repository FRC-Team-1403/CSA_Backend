use crate::comp::parse::TeamYearAroundJsonParser;
use crate::comp::shared::{
    avg, check_win, compare_highest, compare_lowest, deviation, get_breakdown_data, BreakDown, Team,
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
    pub fn low_and_high_and_add(&mut self, compare: Option<i16>) {
        let Some(compare) = compare else {
            return;
        };
        self.highest(compare);
        self.lowest(compare);
        self.add(compare);
    }
    fn highest(&mut self, compare: i16) {
        self.highest = Some(compare_highest(self.highest.unwrap_or_default(), compare))
    }
    fn lowest(&mut self, compare: i16) {
        self.lowest = Some(compare_lowest(self.lowest.unwrap_or(1000), compare));
    }
    fn add(&mut self, what: i16) {
        self.graph.push(what);
    }
    pub fn avg(&mut self) {
        self.avg = Some(avg(self.graph.clone()))
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
    pub fn avg(&mut self) {
        self.avg = avg(self.graph.clone())
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
    pub auto_game_pieces: OData,
    pub telop_game_pieces: OData,
    pub auto_game_points: OData,
    pub telop_game_points: OData,
    pub deviation: f32,
    pub ekam_ai: f32,
    pub contributed_score: f32,
}

impl YearAround {
    pub fn new(data: TeamYearAroundJsonParser) -> Self {
        Self {
            contributed_score: 6.9,
            deviation: 0.0,
            ekam_ai: 6.9,
            data: Some(data),
            points: Data::new(),
            wins: 0,
            losses: 0,
            win_rato: 0.0,
            matches_played: 0,
            pen: OData::new(),
            rp: OData::new(),
            auto: OData::new(),
            auto_game_pieces: OData::new(),
            telop_game_pieces: OData::new(),
            auto_game_points: OData::new(),
            telop_game_points: OData::new(),
        }
    }
    //noinspection ALL
    pub fn calculate(mut self, team: &str) -> Result<Self, std::fmt::Error> {
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
                self.handle(json.alliances.red.score, breakdown);
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
                self.handle(json.alliances.blue.score, breakdown);
            }
        }
        self.data = None;
        self.deviation = deviation(&self.points.graph);
        self.rp.avg();
        self.pen.avg();
        self.points.avg();
        self.auto.avg();
        self.auto_game_points.avg();
        self.auto_game_pieces.avg();
        self.telop_game_points.avg();
        self.telop_game_pieces.avg();
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

    fn handle(&mut self, score: i16, data: BreakDown) {
        //happens if match breakdown works
        if data.auto_points.is_some() {
            //calc data
            self.rp.low_and_high_and_add(data.rp);
            self.auto.low_and_high_and_add(data.auto_points);
            self.pen.low_and_high_and_add(data.foul_points);
            self.auto_game_points.low_and_high_and_add(data.rp);
            self.auto_game_pieces.low_and_high_and_add(data.rp);
            self.telop_game_points
                .low_and_high_and_add(data.telop_game_piece_points);
            self.telop_game_pieces
                .low_and_high_and_add(data.telop_game_piece_count);
        }
        self.points.highest = compare_highest(self.points.highest, score);
        //lowest code
        self.points.lowest = compare_lowest(self.points.lowest, score);
        //adding of avg
        self.points.graph.push(score);
    }
}
