use std::collections::HashMap;
use std::f32::consts::E;
use crate::db::firebase::Firebase;
use crate::http::event::get::get;
use crate::http::event::math::math;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;

pub mod get;
pub mod math;


struct Event{
    pub cache: TeamYearAroundJsonParser
}

impl Event {
    pub fn new() -> Self {
        Self {
            cache: TeamYearAroundJsonParser
        }
    }
    pub async fn set_match(self, team : &str) -> Result<Self,Self> {
        let Ok(json) = get().await else {
            return Err(Self);
        };
        if json == self.cache {
            return Ok(self)
        }
        let Ok(final_data) = math(json, team) else {
            return Err(self)
        };
        todo!();
    }
}