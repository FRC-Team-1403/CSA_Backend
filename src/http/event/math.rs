use std::fmt::Error;
use crate::http::year_around::fuctions::parse::TeamYearAroundJsonParser;
use crate::http::year_around::math::YearAround;

pub fn math(data : TeamYearAroundJsonParser, team: &str ) -> Result<YearAround, Error> {
    YearAround::new(data).calculate(team)
}