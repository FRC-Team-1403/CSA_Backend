use std::{io::Error, process::Command};

pub fn get(args: Vec<String>) -> Result<String, Error> {
    let command = Command::new("microService/firestore_get/bin")
        .args(args)
        .output()?;
    let std_out = String::from_utf8(command.stdout).unwrap();
    let std_err = String::from_utf8(command.stderr).unwrap();
    if std_out.is_empty() {
        return Ok(std_out);
    }
    Ok(std_err)
}
