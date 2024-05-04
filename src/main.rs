mod file_io;
mod helpers;
mod json_parser;
mod schedule_generator;
mod structs;

use crate::file_io::read_json_file;
use crate::helpers::{draw_schedule, filter_classes, generate_periods};
use crate::json_parser::parse_json;
use crate::schedule_generator::generate_schedules;

fn main() {
    let json_str = read_json_file("data.json").unwrap_or_else(|err| {
        eprintln!("Error reading JSON file: {}", err);
        std::process::exit(1);
    });

    let classes = parse_json(&json_str).unwrap_or_else(|err| {
        eprintln!("Error parsing JSON: {}", err);
        std::process::exit(1);
    });

    let subject_codes = ["BOBA11", "861305", "866103"];
    let days = [2, 3, 4, 5, 6, 7];

    let filtered_classes = filter_classes(&classes, &subject_codes, &days);

    let periods = generate_periods(2, 6, 1, 10);

    let schedules = generate_schedules(&filtered_classes, &subject_codes, &periods);

    println!("Possible schedules:");

    for (i, schedule) in schedules.iter().enumerate() {
        println!("Schedule {}:", i + 1);
        draw_schedule(schedule);
    }
}
