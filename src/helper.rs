use crate::error::ConfigError;
use crate::Config;
use time::OffsetDateTime;

use serde_json::from_reader;

use std::{fs, io, io::BufReader, path::Path};

pub fn get_config() -> Result<Config, ConfigError> {
    let path = Path::new("./config.json");
    if !path.exists() {
        return Err(ConfigError::MissingConfig());
    }
    let file = fs::OpenOptions::new()
        .append(true)
        .read(true)
        .create(true)
        .open(path)?;
    let reader = BufReader::new(file);
    let config: Config = from_reader(reader)?;

    Ok(config)
}

fn write_err_log(message: &str, error: String) -> Result<(), io::Error> {
    let date = OffsetDateTime::now_local().unwrap();
    let err_content = format!("[{}\nerror: {}\ndate: {}]\n", message, error, date);

    let path = "./error.log";
    fs::write(path, err_content)
}

pub fn prevent_exit() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => write_err_log("Failed to exit properly.", e.to_string()).unwrap(),
    }
}
