use std::fs::{File, read_dir};
use std::io::{Error, prelude::*};

use std::convert::AsRef;
use std::path::Path;
use std::fmt::Debug;

pub fn open_file<P: AsRef<Path> + Debug + Copy>(path: P) -> Result<File, Error> {
    File::open(path)
}

pub fn read_file<P: AsRef<Path> + Debug + Copy>(path: P) -> String {
    match open_file(path) {
        Ok(mut file) => {
            let file_size = file.metadata().expect("Unable to get file metadata").len() as usize;
            let mut file_content = String::with_capacity(file_size);
            file.read_to_string(&mut file_content).expect("Error reading file");

            file_content
        },
        Err(e) => {
            println!("Unable to open file at path: {:?}\nError: {}", path, e);
            "".to_owned()
        },
    }
}

pub fn read_file_bytes<P: AsRef<Path> + Debug + Copy>(path: P) -> Vec<u8> {
    match open_file(path) {
        Ok(mut file) => {
            let file_size = file.metadata().expect("Unable to get file metadata").len() as usize;
            let mut file_content = Vec::with_capacity(file_size);
            file.read_to_end(&mut file_content).expect("Error reading file");

            file_content
        },
        Err(e) => {
            println!("Unable to open file at path: {:?}\nError: {}", path, e);
            return vec![]
        },
    }
}

pub fn get_watched_cartridges(path: &str) -> Vec<String> {
    read_dir(path).unwrap()
        .into_iter()
        .filter_map(|entry| entry.unwrap().file_name().into_string().ok())
        .collect::<Vec<String>>()
}
