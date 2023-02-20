use crate::comp::event::math::EventData;
use crate::ram::ENV;
use log::warn;
use rayon::prelude::*;
use std::io;
use std::process::Command;

pub struct MatchStore {
    data: Vec<EventData>,
}

impl MatchStore {
    pub fn new(data: Vec<EventData>) -> Self {
        Self { data }
    }
    pub fn send(self) -> Result<(), io::Error> {
        self.data
            .par_iter()
            .try_for_each(|raw_json| -> Result<(), io::Error> {
                let json = serde_json::to_string(&raw_json)?;
                let firestore_location = &ENV.firestore_collection;
                let result = Command::new("microService/firestore_send/bin")
                    .args([
                        json,
                        firestore_location.to_owned(),
                        raw_json.team.to_string(),
                        "Matches".to_owned(),
                        raw_json.match_number.to_string(),
                    ])
                    .output()?;
                let uft8_output = String::from_utf8(result.clone().stdout).unwrap_or(String::new());
                if uft8_output.is_empty() {
                    warn!(
                        "{}",
                        String::from_utf8(result.stderr).unwrap_or("Utf8 error".to_owned())
                    );
                }
                println!("{uft8_output}",);
                Ok(())
            })?;
        Ok(())
    }
}
