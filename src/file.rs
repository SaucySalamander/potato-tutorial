use std::fs::{File, OpenOptions};
use std::io::{Result, Read, Write};

pub fn read_file_to_str(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &str, contents: &str) -> Result<bool> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(true)
}

pub fn append_file(path: &str, contents: &str) -> Result<()>{
    let mut file = OpenOptions::new().append(true).open(path).expect("Failed to open file");
    file.write_all(contents.as_bytes())
}