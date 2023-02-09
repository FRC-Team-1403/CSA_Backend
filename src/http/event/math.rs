use crate::http::event::Event;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;
use crate::http::year_around::math::YearAround;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::Error;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    data: String,
}
impl Event {
    pub fn math(data: TeamYearAroundJsonParser, team: &str) -> Result<YearAround, Error> {
        todo!()
    }
}
