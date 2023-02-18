use std::{io::Error, process::Command};

pub fn firestore_send(json: Result<String, Error>, args: Vec<String>) -> Result<String, Error> {
    let result = Command::new("microService/firestore_send/bin")
        .arg(json?)
        .args(args)
        .output()?;
    let uft8_output = String::from_utf8(result.clone().stdout).unwrap_or(String::new());
    if uft8_output.trim().is_empty() {
        return Ok(String::from_utf8(result.clone().stderr).unwrap_or("Utf8 error".to_owned()));
    }
    Ok(uft8_output)
}
