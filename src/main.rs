mod cli;
mod build;

use clap::{Arg, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = DayRegistry::new();

    let matches = Command::new("Advent of Code 2024")
        .version("1.0")
        .about("Run Advent of Code Solution")
        .arg(
            Arg::new("day")
                .help("Day number (eg., 01, 02)")
                .required(true)
                .index(2),
        )
        .get_matches();

    let day = matches.get_one::<String>("day").expect("day is required");
    let part = matches
        .get_one::<String>("part")
        .expect("Part is required")
        .parse::<u8>()
        .expect("Part must be a number");

    match registry.solve(day, part) {
        Ok(result) => {
            println!("Day {} Part {} Result: {}", day, part, result);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}
