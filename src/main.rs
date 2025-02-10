use std::env;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 2 {
        match (args[0].as_str(), args[1].as_str()) {
            ("day01", "01") => solutions::day01::part1::solve(),
            ("day01", "02") => solutions::day01::part2::solve(),
            _ => println!("Input a solution"),
        }
    }
}
