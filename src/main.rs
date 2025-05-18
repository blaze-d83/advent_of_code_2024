use std::env;

mod solutions;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 2 {
        match (args[0].as_str(), args[1].as_str()) {
            ("day01", "01") => solutions::day01::part01(),
            ("day01", "02") => solutions::day01::part02(),
            ("day02", "01") => solutions::day02::part01(),
            ("day02", "02") => solutions::day02::part02(),
            ("day03", "01") => solutions::day03::part01(),
            ("day03", "02") => solutions::day03::part02(),
            ("day04", "01") => solutions::day04::part01(),
            ("day04", "02") => solutions::day04::part02(),
            ("day05", "01") => solutions::day05::part01(),
            ("day05", "02") => solutions::day05::part02(),
            ("day06", "01") => solutions::day06::part01(),
            ("day06", "02") => solutions::day06::part02(),
            ("day07", "01") => solutions::day07::part1(),
            ("day07", "02") => solutions::day07::part02(),
            ("day08", "01") => solutions::day08::part1(),
            // ("day08", "02") => solutions::day08::part2(),

            _ => println!("Input a solution"),
        }
    }
}
