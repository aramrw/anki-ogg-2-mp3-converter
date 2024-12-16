mod error;
mod helper;
use crate::error::ConfigError;
use std::{
    fs::{self},
    io::{self, Error, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Output},
    thread::sleep,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::os::windows::process::CommandExt;
use winapi::um::winbase::CREATE_NO_WINDOW;
//const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    collections_path: String,
    anki_folder: String,
    display_terminal: bool,
    max_processes: u8,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_secondary_instance = args.len() > 1 && args[1] == "secondary";
    let mut start = Instant::now();

    let mut ogg_file_paths: Vec<String> = Vec::new();

    match collect_ogg_files() {
        Ok(ogg_files) => ogg_file_paths = ogg_files,
        Err(err) => {
            eprintln!("Err attempting to collect ogg files: {}", err);
        }
    };

    let tasks_per_chunk = std::cmp::min(1, num_cpus::get().try_into().unwrap());

    // first check if user has enabled leaving the window open for debugging
    if !is_secondary_instance
    /* && !config.display_terminal */
    {
        std::process::Command::new("convert-ogg-mp3.exe")
            .arg("secondary")
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .unwrap();

        // exit the primary instance once the windowless instance is open.
        std::process::exit(0);
    }

    let mut child: Option<Child> = match launch_anki() {
        Ok(c) => Some(c),
        Err(e) => {
            eprintln!("Failed to Launch Anki.exe: {}", e);
            None
        }
    };

    if !ogg_file_paths.is_empty() {
        convert_for_loop(ogg_file_paths, start, tasks_per_chunk).await;
    } else {
        println!("\nNo .ogg files found.\n");
    }

    if let Some(mut c) = child {
        // Wait for all conversions then check anki status
        let status = c.wait().unwrap();
        // if program reaches here anki has exited, convert ogg files one more time
        let mut final_ogg_files: Vec<String> = Vec::new();

        match collect_ogg_files() {
            Ok(ogg_files) => final_ogg_files = ogg_files,
            Err(err) => {
                eprintln!("Err attempting to collect ogg files: {}", err);
            }
        }

        start = Instant::now();

        if !final_ogg_files.is_empty() {
            convert_for_loop(final_ogg_files, start, tasks_per_chunk).await;
        } else {
            println!("\nThere are no ogg files to convert.\n");
        }

        // let the user see all program messages before exiting
        println!("\n\nAnki has exited, shutting down in 2 seconds....");
        sleep(Duration::from_secs(2));

        std::process::exit(status.code().unwrap_or(1));
    }
}

async fn convert_for_loop(ogg_files: Vec<String>, start: Instant, tasks_per_chunk: u8) {
    let ogg_file_len = ogg_files.len();
    for (index, chunk) in ogg_files.chunks(tasks_per_chunk.into()).enumerate() {
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
        println!(
            "\n\nSuccessfully converted {} ogg files in {} seconds!\n\n",
            ogg_file_len, seconds
        );
    } else {
        println!(
            "\n\nSuccessfully converted {} ogg files in {} minutes and {} seconds!\n\n",
            ogg_file_len, minutes, seconds
        );
    }
}

fn launch_anki() -> Result<Child, ConfigError> {
    let config = helper::get_config()?;
    let anki_exe_path = format!("{}\\anki.exe", config.anki_folder);

    Ok(Command::new(anki_exe_path).spawn()?)
}

fn collect_ogg_files() -> Result<Vec<String>, ConfigError> {
    match helper::get_config() {
        Ok(config) => {
            let mut ogg_files = Vec::new();
            let path = config.collections_path;
            let collection_media_folder = std::fs::read_dir(path)?;

            for entry in collection_media_folder {
                let entry = entry?;
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "ogg" {
                        ogg_files.push(path.to_string_lossy().into_owned());
                    }
                }
            }
            Ok(ogg_files)
        }
        Err(e) => Err(e),
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
            anki_folder: current_dir.to_str().unwrap().to_string(),
            display_terminal: false,
            max_processes: 128,
        };

        Ok(config)
    } else {
        println!("Invalid collection.media folder. Try again.");
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid collection.media folder",
        ))
    }
}

async fn convert_ogg_to_mp3(ogg_path: &str) -> Result<Output, Error> {
    println!("Converting {}", ogg_path);

    let mut output_path = String::new();
    if let Some((left, _right)) = &ogg_path.rsplit_once('.') {
        let joined = format!("{}.mp3", left);
        output_path = joined;
    }
    let output = Command::new("ffmpeg")
        .args(["-i", ogg_path, "-codec:a", "libmp3lame", &output_path])
        .output();

    match output {
        Ok(output) => {
            // delete the ogg file after conversion
            if Path::new(&output_path).exists() {
                fs::remove_file(ogg_path)?;
            }

            Ok(output)
        }
        Err(e) => {
            eprint!("Failed to execute command: {}", e);
            Err(e)
        }
    }
}
