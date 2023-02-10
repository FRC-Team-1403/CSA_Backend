use crate::http::event::math::EventData;
use std::io;
use std::process::Command;

pub struct MatchStore {
    data: Vec<EventData>,
}

impl MatchStore {
    pub fn new(data: Vec<EventData>) -> Self {
        Self { data }
    }
    //todo
    pub fn send(self) -> Result<(), io::Error> {
        let json = serde_json::to_string(&self.data)?;
        let result = String::from_utf8(
            Command::new("microService/firestore_send/bin")
                .arg(json)
                .arg(year_check.to_string())
                .output()?
                .stdout,
        )
        .unwrap_or("utf8 error".to_owned());
        if result.trim() != "success" {
            println!("FAILURE: {result}, skipping that team",)
        }
        Ok(())
    }
}
