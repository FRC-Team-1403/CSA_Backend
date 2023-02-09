use crate::http::event::get::get;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;

pub mod get;
pub mod math;

pub struct Event {
    pub cache: TeamYearAroundJsonParser,
    pub new_data: TeamYearAroundJsonParser,
    team: i16,
}

impl Event {
    // pub fn new(data: TeamYearAroundJsonParser) -> Self {
    //     Self { cache: data }
    // }
    pub async fn set_match(self, team: &str) -> Result<Self, Self> {
        let Ok(json) = get().await else {
            return Err(self);
        };
        if json == self.cache {
            return Ok(self);
        }
        // let Ok(_final_data) = self.math() else {
        //     return Err(self)
        // };
        todo!();
    }
}
