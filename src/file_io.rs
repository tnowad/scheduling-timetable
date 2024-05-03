use std::{
    fs::File,
    io::{self, Read},
};

pub fn read_json_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut json_str = String::new();
    file.read_to_string(&mut json_str)?;
    Ok(json_str)
}
