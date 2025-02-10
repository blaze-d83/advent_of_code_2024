use std::env;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 2 {
        match (args[0].as_str(), args[1].as_str()) {
            ("day01", "01") => {
                if let Err(e) = solutions::day01::part1::solve() {
                    eprintln!("Error: {}", e);
                }
            }
            // ("day01", "02") => {
            //     if let Err(e) = solutions::day01::part2::solve() {
            //         eprintln!("Error: {}", e);
            //     }
            // }
            _ => println!("Input a solution"),
        }
    }
}
