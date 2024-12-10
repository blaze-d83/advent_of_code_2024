use clap::{Arg, Command};
use std::collections::HashMap;

mod day01;

trait DaySolution {
    fn solve() -> String;
}

macro_rules! register_solution {
    ($registry:expr, $day:expr, $part:expr, $module:path) => {
        $registry.solutions.insert(
            format!("{}_{}", $day, $part),
            Box::new(|| <$module>::solve()),
        );
    };
}

struct DayRegistry {
    solutions: HashMap<String, Box<dyn Fn() -> String>>,
}

impl DayRegistry {
    fn new() -> Self {
        let mut registry = DayRegistry {
            solutions: HashMap::new(),
        };

        register_solution!(registry, "01", 1, day01::part1);

        registry
    }

    fn solve(&self, day: &str, part: u8) -> Result<String, String> {
        let key = format!("{}_{}", day, part);
        self.solutions
            .get(&key)
            .map(|solver| solver())
            .ok_or_else(|| format!("No solution found for Day {} Part {}", day, part))
    }
}

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
