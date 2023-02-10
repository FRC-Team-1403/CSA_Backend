use crate::http::event::get::get;
use crate::http::event::math::EventData;
use crate::http::shared::Shared;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;

pub mod get;
pub mod math;

pub struct Event {
    pub cache: TeamYearAroundJsonParser,
    pub new_data: TeamYearAroundJsonParser,
    team: u16,
}

impl Event {
    pub fn new(team: u16) -> Self {
        Self {
            cache: vec![],
            new_data: vec![],
            team,
        }
    }
    pub async fn send_request(mut self) -> Result<Self, Self> {
        let Ok(json) = get().await else {
            return Err(self);
        };
        self.new_data = json;
        if self.new_data == self.cache {
            return Err(self);
        }
        Ok(self)
    }
    pub fn parse(self) -> Result<(Self, Vec<EventData>), Self> {
        let Ok(final_data) = self.math() else {
            return Err(self)
        };
        Ok((self, final_data))
    }
    pub async fn update_match_data(mut self) -> Self {
        let teams = Event::team();
        match self.send_request().await {
            Ok(class) => self = class,
            Err(e) => return e,
        }
        for team in teams {
            self.team = team;
            let event_data;
            match self.parse() {
                Ok((class, event)) => {
                    self = class;
                    event_data = event;
                }
                Err(e) => return e,
            }
            dbg!(event_data);
        }
        self
    }
}
