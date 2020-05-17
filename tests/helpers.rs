extern crate serde;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    hostname: String,
    username: String,
    password: String,
    pub cartridges_path: String,
    cartridges: Option<Vec<String>>,
    ignore_list: Option<Vec<String>>,
    code_version: String,
}

fn read_file<P: AsRef<Path> + Debug + Copy>(path: P) -> Result<String, String> {
    match File::open(path) {
        Ok(mut file) => {
            let file_size = file.metadata().expect("Unable to get file metadata").len() as usize;
            let mut file_content = String::with_capacity(file_size);
            file.read_to_string(&mut file_content).expect("Error reading file");

            Ok(file_content)
        },
        Err(e) => Err(format!("Unable to open file at path: {:?}\nError: {}", path, e)),
    }
}

fn save_file<P: AsRef<Path> + Debug + Copy>(path: P, data: &Config) -> Result<(), String> {
    let full_file_path: PathBuf = get_full_path(path);

    match serde_json::to_string(data) {
        Ok(string) => {
            match File::create(full_file_path.as_path()) {
                Ok(mut file) => {
                    match file.write_all(string.into_bytes().as_ref()) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(format!("{0}", e)),
                    }
                },
                Err(e) => Err(format!("{0}", e)),
            }
        },
        Err(e) => Err(format!("{0}", e))
    }
}

fn get_config<P: AsRef<Path> + Debug + Copy>(path: P) -> Result<Config, String> {
    match read_file(path) {
        Ok(json_string) => {
            let config: Config = serde_json::from_str(json_string.as_str()).expect("Unable to parse json");
            Ok(config)
        },
        Err(e) => Err(e),
    }
}

pub fn get_full_path<P: AsRef<Path> + Debug + Copy>(local_path: P) -> PathBuf {
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push(local_path);

    full_path
}

pub fn create_json_file() -> Result<(), String> {
    let full_file_path: PathBuf = get_full_path("tests/config/config.json");
    // get config
    let mut config: Config = get_config(full_file_path.as_path())?;
    let full_cartridges_path: String = get_full_path(AsRef::<str>::as_ref(config.cartridges_path.as_str())).into_os_string().to_str().unwrap().to_owned();
    // update cartridges path
    config.cartridges_path = full_cartridges_path;
    // save new dw.json config
    save_file("tests/config/dw.json", &config)?;
    Ok(())
}