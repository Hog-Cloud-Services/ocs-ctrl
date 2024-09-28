use std::{error::Error, fmt::Display, fs::{self, read}, path::{self, PathBuf}, str::FromStr};

use crate::config;

// General Error implementation for all IO related file operations
#[derive(Debug)]
pub struct FileOpError {
    message: String
}

impl FromStr for FileOpError {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(FileOpError{message: s.to_owned()})
    }
}

impl Display for FileOpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(&self.message);
        Ok(())
    }
}

impl From<std::io::Error> for FileOpError {
    fn from(value: std::io::Error) -> Self {
        let message = value.to_string();     
        return Self { message }
    }
}

impl Error for FileOpError {}

fn parse_file_path(bucket: &str, file_name: &str) -> PathBuf {
    let mut final_path = path::PathBuf::from_str(&config::ROOT_SAVE_PATH).unwrap();
    final_path.push(bucket);
    let parsed_file_name = fs::canonicalize(file_name).unwrap();
    final_path.push(parsed_file_name);
    final_path
}

pub fn get_file_content(bucket: &str, file_name: &str) -> Result<Vec<u8>, FileOpError>{
    let destination_path = parse_file_path(bucket, file_name);
    let content = fs::read(destination_path)?;
    Ok(content)
}

pub fn write_file_content(bucket: &str, file_name: &str, data: &[u8]) -> Result<(), FileOpError> {
    let destination_path = parse_file_path(bucket, file_name);
    fs::write(destination_path, data)?;
    Ok(())
}

pub fn file_exists(bucket: &str, file_name: &str) -> bool {
    let destination_path = parse_file_path(bucket, file_name);
    destination_path.as_path().exists()
}
