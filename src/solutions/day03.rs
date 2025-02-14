use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn part01() {
    let file_path = "inputs/day03.txt";
    match read_input_file(file_path) {
        Ok(data) => {
            let result = mul_by_regex(data);
            println!("The sum of all mul values: {}", result);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn part02() {
    let file_path = "inputs/day03.txt";
    match read_input_file(file_path) {
        Ok(data) => {
            let result = dos_and_donts(data);
            println!("Result: {}", result);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn dos_and_donts(data: Vec<String>) -> u64 {
    let input = data.join("\n");

    let re = Regex::new(
        r"(?P<do>do\(\)|undo\(\))|(?P<dont>don't\(\))|(?P<mul>mul\(\s*(?P<a>\d+)\s*,\s*(?P<b>\d+)\s*\))"
    ).unwrap();

    let mut enabled = true;
    let mut sum: u64 = 0;

    for cap in re.captures_iter(&input) {
        if cap.name("do").is_some() {
            enabled = true;
        } else if cap.name("dont").is_some() {
            enabled = false;
        } else if cap.name("mul").is_some() {
            if enabled {
                let a: u64 = cap.name("a").unwrap().as_str().parse().unwrap();
                let b: u64 = cap.name("b").unwrap().as_str().parse().unwrap();
                sum += a * b;
            }
        }
    }
    sum
}

fn mul_by_regex(data: Vec<String>) -> u64 {
    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    data.iter()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| {
                    let a: u64 = cap[1].parse().unwrap();
                    let b: u64 = cap[2].parse().unwrap();
                    a * b
                })
                .sum::<u64>()
        })
        .sum()
}

fn read_input_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}
