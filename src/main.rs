mod file_io;
mod json_parser;
use crate::file_io::read_json_file;
use crate::json_parser::parse_json;

fn main() {
    let json_str = read_json_file("data.json").unwrap_or_else(|err| {
        eprintln!("Error reading JSON file: {}", err);
        std::process::exit(1);
    });

    let classes = parse_json(&json_str).unwrap_or_else(|err| {
        eprintln!("Error parsing JSON: {}", err);
        std::process::exit(1);
    });

}
