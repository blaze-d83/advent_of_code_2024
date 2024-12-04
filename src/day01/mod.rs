use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/day01/input.txt").expect("Failed to read input");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    0
}

fn part2(input: &str) -> i32 {
    0
}
