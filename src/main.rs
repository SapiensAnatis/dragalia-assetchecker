extern crate glob;
mod config;

use crate::config::Config;
use data_encoding::BASE32;
use glob::glob;
use sha2::{Digest, Sha256};
use std::{ffi::OsStr, fs, path::PathBuf};

fn main() {
    println!("Checking assets...");

    let data = fs::read_to_string("./config.json").expect("Failed to read config.json");
    let config: Config = serde_json::from_str(&data).expect("Failed to deserialize config.json");

    for asset_folder in config.assetpaths {
        let glob_pattern = format!("{}/**/*", asset_folder);
        let paths = glob(&glob_pattern).expect("Failed to read glob pattern");

        // todo: add multithreading

        paths
            .into_iter()
            .filter_map(|p| match p {
                Ok(value) => match value.is_file() {
                    true => Some(value),
                    false => None,
                },
                Err(_) => None,
            })
            .enumerate()
            .for_each(|(_, path)| {
                match check_asset(&path) {
                    true => (),
                    false => println!("fail: {:?}", path),
                }
            });
    }

    println!("Done!")
}

fn check_asset(path: &PathBuf) -> bool {
    let mut hasher = Sha256::new();

    match fs::read(path) {
        Ok(file) => {
            hasher.update(&file);
            let content_bytes = hasher.finalize();
            let hash_name: String = BASE32.encode(&content_bytes).chars().take(52).collect();
            let file_name = path.file_name().expect("Failed to read filename");

            OsStr::new(&hash_name) == file_name
        }
        Err(e) => {
            println!("File read failure: {:?}", e);
            false
        }
    }
}
