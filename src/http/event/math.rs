use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;
use crate::http::year_around::math::YearAround;
use std::fmt::Error;

pub fn math(data: TeamYearAroundJsonParser, team: &str) -> Result<YearAround, Error> {
    YearAround::new(data).calculate(team)
}
