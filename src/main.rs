fn main() {
    let json_str = read_json_file("data.json").unwrap_or_else(|err| {
        eprintln!("Error reading JSON file: {}", err);
        std::process::exit(1);
    });

}
