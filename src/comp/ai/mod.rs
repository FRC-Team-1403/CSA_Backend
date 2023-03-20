use super::avg::math::YearAround;
use super::shared::avg;
use crate::comp::shared::deviation;
use crate::ram::get_pub;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use log::{debug, info};
use plr::regression::OptimalPLR;
use std::error::Error;

pub enum Type<'a> {
    Match(&'a u16),
    Year,
}

pub struct Ai {}

impl Ai {
    pub fn predict(data: &Vec<i16>) -> Result<(), Box<dyn Error>> {
        // everything above 6.5 is considered a good wine
        let (train, valid) = linfa_datasets::winequality()
            .map_targets(|x| if *x > 6 { "good" } else { "bad" })
            .split_with_ratio(0.9);

        println!(
            "Fit Logistic Regression classifier with #{} training points",
            train.nsamples()
        );

        // fit a Logistic regression model with 150 max iterations
        let model = LogisticRegression::default()
            .max_iterations(150)
            .fit(&train)
            .unwrap();

        // predict and map targets
        let pred = model.predict(&valid);

        // create a confusion matrix
        let cm = pred.confusion_matrix(&valid).unwrap();

        // Print the confusion matrix, this will print a table with four entries. On the diagonal are
        // the number of true-positive and true-negative predictions, off the diagonal are
        // false-positive and false-negative
        println!("{:?}", cm);

        // Calculate the accuracy and Matthew Correlation Coefficient (cross-correlation between
        // predicted and targets)
        println!("accuracy {}, MCC {}", cm.accuracy(), cm.mcc());

        Ok(())
    }

    fn slope(vals: &Vec<i16>) -> bool {
        if vals.len() < 6 {}
        let data_points: Vec<(f64, f64)> = vals
            .iter()
            .enumerate()
            .map(|(index, val)| (index.to_owned() as f64, val.to_owned() as f64))
            .collect();
        let mut plr = OptimalPLR::new(0.05);
        let mut segments = Vec::new();
        for (x, y) in data_points {
            if let Some(segment) = plr.process(x, y) {
                segments.push(segment);
            }
        }
        if let Some(slope) = plr.finish() {
            return slope.slope > 0.0;
        }
        false
    }
    fn avg_regession(vals: Vec<i16>) -> f32 {
        let calc = {
            if vals.len() > 5 {
                let parse = vals.len() - 5;
                vals[parse..].to_owned()
            } else {
                vals
            }
        };
        avg(calc)
    }
    pub fn calc_v2(match_data: &YearAround, what: Type) -> i32 {
        if match_data.points.graph.len() < 2 {}
        todo!()
    }
    pub fn calc_v1(match_data: &YearAround, what: Type) -> f32 {
        let match_br = Self::calc_avg_br(match_data);
        let year_br: f32 = {
            match what {
                Type::Match(team) => {
                    if let Some(year) = get_pub().get(team) {
                        Self::calc_avg_br(year)
                    } else {
                        Self::calc_avg_br(match_data)
                    }
                }
                Type::Year => Self::calc_avg_br(match_data),
            }
        };
        match_br + (year_br / 1.5)
    }
    fn math_v2(avg_points: f32, win_ratio: f32, points_graph: &Vec<i16>) -> f32 {
        let add = {
            if Self::slope(points_graph) {
                5.0
            } else {
                0.0
            }
        };
        let val = ((avg_points * 2.0 + Self::avg_regession(points_graph.to_owned()) / 3.0) / 2.5)
            + (win_ratio * 10.0)
            + add
            - (deviation(points_graph) * 1.5);
        debug!("Ai val is: {}", val);
        val
    }
    fn math_v1(
        avg_points: f32,
        win_ratio: f32,
        rp: f32,
        penalty: f32,
        points_graph: &Vec<i16>,
    ) -> f32 {
        let val = ((avg_points + Self::avg_regession(points_graph.to_owned()) / 2.0) / 2.5)
            + (win_ratio * 10.0)
            + (rp)
            - ((penalty / 2.0) + (deviation(points_graph) / 3.2));
        debug!("Ai val is: {}", val);
        val
    }

    fn calc_avg_br(year: &YearAround) -> f32 {
        let penalty: f32;
        let rp: f32;
        if let Some(pen) = year.pen.avg {
            penalty = pen;
        } else {
            penalty = 0.0;
        }
        if let Some(rp_data) = year.rp.avg {
            rp = rp_data;
        } else {
            rp = 0.0;
        }
        Self::math_v1(
            year.points.avg,
            year.win_rato,
            rp,
            penalty,
            &year.points.graph,
        )
    }
}
