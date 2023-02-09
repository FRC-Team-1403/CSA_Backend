use crate::http::event::get::get;
use crate::http::event::math::math;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;

pub mod get;
pub mod math;

struct Event {
    pub cache: TeamYearAroundJsonParser,
}

impl Event {
    pub fn new(data: TeamYearAroundJsonParser) -> Self {
        Self { cache: data }
    }
    pub async fn set_match(self, team: &str) -> Result<Self, Self> {
        let Ok(json) = get().await else {
            return Err(self);
        };
        if json == self.cache {
            return Ok(self);
        }
        let Ok(_final_data) = math(json, team) else {
            return Err(self)
        };
        todo!();
    }
}
