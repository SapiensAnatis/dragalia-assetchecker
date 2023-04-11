extern crate glob;
mod config;

use crate::config::Config;
use data_encoding::BASE32;
use glob::glob;
use sha2::{Digest, Sha256};
use std::{ffi::OsStr, fs, path::PathBuf};

fn main() {
    let data = fs::read_to_string("./config.json").expect("Failed to read config.json");
    let config: Config = serde_json::from_str(&data).expect("Failed to deserialize config.json");

    for asset_folder in config.assetpaths {
        println!("Checking asset folder {}", asset_folder);
        let glob_pattern = format!("{}/**/*", asset_folder);

        let paths = glob(&glob_pattern).expect("Failed to read glob pattern");
        let n_paths = glob(&glob_pattern)
            .expect("Failed to read glob pattern")
            .count();

        paths
            .into_iter()
            .filter_map(|p| match p {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .filter(|p| p.is_file())
            .enumerate()
            .for_each(|(idx, path)| {
                let percent = format!("{:.1}%", (idx as f32 / n_paths as f32) * 100.0);
                print!("Checking path {:?} ({} / {}, {})\r", path, idx, n_paths, percent);

                match check_asset(&path) {
                    true => (),
                    false => println!("\nAsset path {:?} failed verification!", path),
                }
            });
    }
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
