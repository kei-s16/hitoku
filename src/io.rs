use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> Result<String, String> {
    if !Path::new(path).exists() {
        return Err("Template file not found.".to_string())
    }

    let mut file_body = String::new();

    let mut file_reader = File::open(path).unwrap();
    file_reader
        .read_to_string(&mut file_body)
        .expect("something went wrong reading the file");

    Ok(file_body)
}

pub fn write_file(path: &str, content: &str) -> Result<(), String> {
    if Path::new(path).exists() {
        return Err("A file with the same name already exists.".to_string())
    }

    let mut file = File::create(path).unwrap();
    let _ = file.write_all(content.as_bytes());

    Ok(())
}
