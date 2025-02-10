use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn part01() -> Result<i32, Box<dyn std::error::Error>> {
    let (left_str, right_str) = read_input_file("inputs/day01_part1.txt")?;
    let (left, right) = cleaned_input(&left_str, &right_str)?;

    let total_distance = calculate_total_distance(&left, &right);
    println!("Total distance: {}", total_distance);
    Ok(total_distance)
}

pub fn part02() -> Result<i32, Box<dyn std::error::Error>> {
    let (left_str, right_str) = read_input_file("inputs/day01_part1.txt")?;
    let (left, right) = cleaned_input(&left_str, &right_str)?;
    Ok(0)
}

fn read_input_file(path: &str) -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut left_strings = Vec::new();
    let mut right_strings = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            left_strings.push(parts[0].to_string());
            right_strings.push(parts[1].to_string());
        }
    }
    Ok((left_strings, right_strings))
}

fn cleaned_input(left: &[String], right: &[String]) -> Result<(Vec<i32>, Vec<i32>), ParseIntError> {
    // Convert strings to i32
    let left_int = convert_to_i32(&left)?;
    let right_int = convert_to_i32(&right)?;

    // Sort the values
    let mut left_sorted = left_int.clone();
    let mut right_sorted = right_int.clone();

    left_sorted.sort();
    right_sorted.sort();

    Ok((left_sorted, right_sorted))
}

fn convert_to_i32(values: &[String]) -> Result<Vec<i32>, ParseIntError> {
    values.iter().map(|s| s.parse::<i32>()).collect()
}

fn calculate_total_distance(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

// fn calculate_similarity_score(left: &[i32], right: &[i32]) -> i32 {
//     1
// }
