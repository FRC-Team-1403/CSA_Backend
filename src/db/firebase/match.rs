use crate::comp::event::math::EventData;
use crate::ram::ENV;
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
        for raw_json in self.data {
            let json = serde_json::to_string(&raw_json)?;
            let firestore_location = &ENV.firestore_collection;
            let result = String::from_utf8(
                Command::new("microService/firestore_send/bin")
                    .args([
                        json,
                        firestore_location.to_owned(),
                        raw_json.match_number.to_string(),
                    ])
                    .output()?
                    .stdout,
            )
            .unwrap_or("utf8 error".to_owned());
            if result.trim() != "success" {
                println!("FAILURE: {result}, skipping that team",)
            }
        }
        Ok(())
    }
}
