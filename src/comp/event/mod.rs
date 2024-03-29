use crate::comp::event::math::EventData;
use crate::comp::http::get_match;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::db::firebase::r#match::MatchStore;
use crate::ram::ENV;

pub mod math;

pub struct Event {
    pub updated: bool,
    pub cache: TeamYearAroundJsonParser,
    pub new_data: TeamYearAroundJsonParser,
}

impl Event {
    pub fn new() -> Self {
        Self {
            updated: false,
            cache: vec![],
            new_data: vec![],
        }
    }
    pub async fn send_request(mut self) -> Result<Self, Self> {
        let Ok(json) = get_match().await else {
            return Err(self);
        };
        if json == self.cache {
            self.updated = false;
            return Err(self);
        }
        self.updated = true;
        self.cache = json.clone();
        self.new_data = json;
        Ok(self)
    }
    pub fn parse(self, team: u16) -> (Self, Vec<EventData>) {
        let final_data = self.math(team);
        (self, final_data)
    }
    pub async fn update_match_data(mut self) -> Self {
        let teams = &ENV.teams;
        match self.send_request().await {
            Ok(class) => self = class,
            Err(e) => return e,
        }
        for team in teams {
            let event_data;
            (self, event_data) = self.parse(team.to_owned());
            MatchStore::new(event_data).send();
        }
        self
    }
}
