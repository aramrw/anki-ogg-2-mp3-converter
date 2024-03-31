use core::fmt;
use std::{
    env, fs::{self, File, OpenOptions}, io::{self, read_to_string, Error, Write}, path::{Path, PathBuf}, process::{Command, Output}, time::Instant
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

impl std::fmt::Display for ConfigErrors {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "ConfigErrors: missing_key: {:?}, file_to_string: {:?}, open_as_file: {:?}, current_dir: {:?}", self.missing_key, self.file_to_string, self.open_as_file, self.current_dir)
     }
}

impl std::error::Error for ConfigErrors {}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let ogg_files = collect_ogg_files().unwrap();
    let tasks_per_chunk = std::cmp::min(10, num_cpus::get());

    for (index, chunk) in ogg_files.chunks(tasks_per_chunk).enumerate() {
        let duration = start.elapsed().as_secs();
        let minutes = duration / 60;
        let seconds = duration % 60;
        if minutes == 0 {
            print!("\r{}: {}s", index, seconds);
        } else {
            print!("\r{}: {}m{}s", index, minutes, seconds);
        }
        io::stdout().flush().unwrap(); 

        let mut tasks = Vec::new();
        for file_path in chunk {
            let file_path = file_path.clone();
            let task = tokio::spawn(async move {
                //println!("{}", file_path);
                
                convert_ogg_to_mp3(&file_path).await.unwrap();
            });
            tasks.push(task);
        }
        futures::future::join_all(tasks).await;
    }

    let duration = start.elapsed().as_secs();
    let minutes = duration / 60;
    let seconds = duration % 60;
        
    if minutes == 0 {
        println!("\n\nSuccessfully converted {} ogg files in {} seconds!", ogg_files.len(), seconds);
    } else {
        println!("\n\nSuccessfully converted {} ogg files in {} minutes and {} seconds!", ogg_files.len(), minutes, seconds);
    }
}

fn collect_ogg_files() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    match check_paths() {
        Ok(config) => {
            let mut ogg_files = Vec::new();
            println!("\n\nReading directory: {}", config.collections_path);
            let path = config.collections_path;
            let collection_media_folder = std::fs::read_dir(&path).unwrap();
            for entry in collection_media_folder {
                let entry = entry?;
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "ogg" {
                        //println!("{:?}", path.clone());
                        ogg_files.push(path.to_string_lossy().into_owned());
                    }
                }
            }
            Ok(ogg_files)
        }
        Err(e) => {
            Err(Box::new(e))
        }
    }
}


fn ask_collection_folder(current_dir: PathBuf) -> Result<Config, io::Error> {
    let mut input = String::new();
    println!("Example media folder: C:\\Users\\exampleUser\\AppData\\Roaming\\Anki2\\user1\\collection.media");
    println!("Enter your collection.media folder path: ");
    io::stdin().read_line(&mut input)?;
    input = input.trim_end().to_string();
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
    let file = OpenOptions::new().append(true).read(true).create(true).open(path);

    match file {
        Ok(mut config_file) => {
            let mut new_config_file: File;
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
                        new_config_file = OpenOptions::new().append(true).read(true).create(true).open(path).unwrap();

                        match read_to_string(&mut new_config_file) {
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
            } else {
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


async fn convert_ogg_to_mp3(ogg_path: &str) -> Result<Output, Error> {
    let mut output_path = String::new();
        if let Some((left, _right)) = &ogg_path.rsplit_once(".") {
            let joined = format!("{}.mp3", left);
            output_path = joined;
        }
    let output = Command::new("ffmpeg")
        .args(&["-i", ogg_path, "-codec:a", "libmp3lame", &output_path])
        .output();

    match output {
        Ok(output) => {
            // delete the ogg file after conversion
            if Path::new(&output_path).exists() {
                fs::remove_file(ogg_path)?;
            }
            
            Ok(output)
        },
        Err(e) => {
            eprint!("Failed to execute command: {}", e);
            return Err(e);
        }
    }
}
