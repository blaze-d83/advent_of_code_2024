use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solve() -> Result<i32, Box<dyn std::error::Error>> {
    // Read the input file
    let (left_strings, right_strings) = read_input_file("inputs/day01_part1.txt")?;

    // Convert strings to i32
    let left_values = convert_to_i32(&left_strings)?;
    let right_values = convert_to_i32(&right_strings)?;

    // Sort the values
    let mut left_sorted = left_values.clone();
    let mut right_sorted = right_values.clone();

    left_sorted.sort();
    right_sorted.sort();

    let total_distance = calculate_total_distance(&left_sorted, &right_sorted);
    println!("Total distance: {}", total_distance);
    Ok(total_distance)
}

fn read_input_file(path: &str) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            left_values.push(parts[0].to_string());
            right_values.push(parts[1].to_string());
        }
    }
    Ok((left_values, right_values))
}

fn convert_to_i32(values: &[String]) -> Result<Vec<i32>, std::num::ParseIntError> {
    values.iter().map(|s| s.parse::<i32>()).collect()
}

fn calculate_total_distance(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}
