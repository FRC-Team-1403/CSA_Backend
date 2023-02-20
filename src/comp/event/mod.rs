use crate::comp::event::math::EventData;
use crate::comp::http::get_match;
use crate::comp::parse::TeamYearAroundJsonParser;
use crate::comp::shared::team;
use crate::db::firebase::r#match::MatchStore;

pub mod math;

pub struct Event {
    pub cache: TeamYearAroundJsonParser,
    pub new_data: TeamYearAroundJsonParser,
}

impl Event {
    pub fn new() -> Self {
        Self {
            cache: vec![],
            new_data: vec![],
        }
    }
    pub async fn send_request(mut self) -> Result<Self, Self> {
        let Ok(json) = get_match().await else {
            return Err(self);
        };
        if json == self.cache {
            return Err(self);
        }
        self.cache = json.clone();
        self.new_data = json;
        Ok(self)
    }
    pub fn parse(self, team: u16) -> Result<(Self, Vec<EventData>), Self> {
        let Ok(final_data) = self.math(team) else {
            return Err(self);
        };
        Ok((self, final_data))
    }
    pub async fn update_match_data(mut self) -> Self {
        let teams = team();
        match self.send_request().await {
            Ok(class) => self = class,
            Err(e) => return e,
        }
        for team in teams {
            let event_data;
            match self.parse(team) {
                Ok((class, event)) => {
                    self = class;
                    event_data = event;
                }
                Err(e) => return e,
            }
            let _ = MatchStore::new(event_data).send();
        }
        self
    }
}
