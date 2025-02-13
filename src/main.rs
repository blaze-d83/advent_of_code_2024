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

            _ => println!("Input a solution"),
        }
    }
}
