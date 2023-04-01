use crate::ram::ENV;
use log::error;
use log::info;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::{io::Error, process::Command};

#[derive(Deserialize, Serialize, Debug)]
pub struct GoReturnVal {
    #[serde(rename = "Average Auto Contributed")]
    pub auto: f32,
    #[serde(rename = "Average Points Contributed")]
    pub total: f32,
    #[serde(rename = "Average Telop Contributed")]
    pub teleop: f32,
    #[serde(rename = "Error")]
    error: String,
}

pub fn get_avg(team: &u16) -> Result<GoReturnVal, Error> {
    let command = Command::new("microService/firestore_send/bin")
        .arg("get")
        .arg(&ENV.firestore_collection)
        .arg(team.to_string())
        .output()?;
    let std = {
        let std_out = String::from_utf8(command.stdout).unwrap();
        let std_err = String::from_utf8(command.stderr).unwrap();
        if std_out.is_empty() {
            std_err
        } else {
            std_out
        }
    };
    let json_go_return_val: serde_json::error::Result<GoReturnVal> = serde_json::from_str(&std);
    match json_go_return_val {
        Ok(json_go_return_val) => {
            if json_go_return_val.error != "Success" {
                error!(
                    "failure to get avg data due to {}",
                    json_go_return_val.error
                );
            }
            info!("Got Data With Repose of {}", json_go_return_val.error);
            Ok(json_go_return_val)
        }
        Err(e) => {
            error!("FIRESTORE GET FAILURE DUE TO: {e}\nWith a value of {std}");
            Err(Error::from(e))
        }
    }
}
