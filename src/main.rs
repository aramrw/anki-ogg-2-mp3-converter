use std::{
    env, fs::OpenOptions, io::{self, read_to_string, Error, Write}, path::{Path, PathBuf}, process::{Command, Output}
};

use serde::{Deserialize, Serialize};
use serde_json::{self, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    collections_path: String,
    anki_folder: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ConfigErrors {
    missing_key: Option<String>,
    file_to_string: Option<String>,
    open_as_file: Option<String>,
    current_dir: Option<String>,
}

fn main() {
   match check_paths() {
    Ok(config) => {
        println!("{:?}", config);
    }
    Err(e) => {
        eprintln!("{:#?}", e);
    }
   }
}

fn ask_collection_folder(current_dir: PathBuf) -> Result<Config, io::Error> {
     let mut input = String::new();
                        println!("Example media folder: C:\\Users\\exampleUser\\AppData\\Roaming\\Anki2\\user1\\collection.media");
                        println!("Enter your collection.media folder path: ");
     io::stdin().read_line(&mut input)?;
                        if !input.is_empty() && input.contains("collection.media") {
                            let config: Config = Config {
                                collections_path: input,
                                anki_folder: current_dir.to_str().unwrap().to_string()
                            };
                            return Ok(config);
                        } else {
                            println!("Invalid collection.media folder. Try again.");
                            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid collection.media folder"));
                        }
}


fn check_paths() -> Result<Config, ConfigErrors> {
    let path = "./config.json";
    let file_exists = Path::new(path).exists();
    let file = OpenOptions::new().append(true).create(true).open(path);

    match file {
        Ok(mut config_file) => {
            if !file_exists {
                match env::current_dir() {
                    Ok(current_dir) => {
                        // If ask_collection_folder() returns an error, it will be propagated upwards
                        let mut config = ask_collection_folder(current_dir.clone());
                        while config.is_err() {
                            config = ask_collection_folder(current_dir.clone());
                        }
                        let config = config.unwrap();
                        let json = to_string_pretty(&config).expect("Failed to serialize config to JSON");
                        config_file.write_all(json.as_bytes()).expect("Failed to write to config.json");
                    },
                    Err(e) => {
                        return Err(ConfigErrors {
                            current_dir: Some(format!("Error getting current_directory: {}", e)),
                            file_to_string: None,
                            missing_key: None,
                            open_as_file: None,
                        });
                    } 
                };
            } 
            match read_to_string(&mut config_file) {
                Ok(config_string) => {
                    let config: Config = serde_json::from_str(&config_string).expect("Fatal: Error converting config_string to json");
                    if !config.anki_folder.is_empty() && !config.collections_path.is_empty() {
                        return Ok(config);
                    } else if !config.anki_folder.is_empty() {
                        return Err(ConfigErrors {
                            file_to_string: None,
                            missing_key: Some("anki_folder key config.json is missing a value.".to_string()),
                            open_as_file: None,
                            current_dir: None, 
                        });
                    } else if !config.collections_path.is_empty() {
                        return Err(ConfigErrors {
                            file_to_string: None,
                            missing_key: Some("collections_path key in config.json is missing a value.".to_string()),
                            open_as_file: None,
                            current_dir: None, 
                        });
                    } else {
                        return Err(ConfigErrors {
                            file_to_string: None,
                            missing_key: Some("anki_folder && collections_path keys in config.json are missing a value.".to_string()),
                            open_as_file: None,
                            current_dir: None, 
                        });
                    }
                }
                Err(e) => {
                    return Err(ConfigErrors {
                        file_to_string: Some(format!("Error reading config_file to config_string: {}", e)),
                        missing_key: None,
                        open_as_file: None,
                        current_dir: None, 
                    });
                }
            }
        }
        Err(e) => {
            return Err(ConfigErrors {
                file_to_string: None,
                missing_key: None,
                open_as_file: format!("Error opening config as File: {}", e).into(),
                current_dir: None,
            });
        }
    }
}


fn convert_ogg_to_mp3(ogg_path: &str, output_path: &str) -> Result<Output, Error> {
    let output = Command::new("ffmpeg")
        .args(&["-i", ogg_path, "-codec:a", "libmp3lame", output_path])
        .output();

    match output {
        Ok(output) => Ok(output),
        Err(e) => {
            eprint!("Failed to execute command: {}", e);
            return Err(e);
        }
    }
}
