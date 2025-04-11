use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
};

pub fn part01() {
    let (left_str, right_str) = match read_input_file("inputs/day01.txt") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let (left, right) = match cleaned_input(&left_str, &right_str) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error cleaning the data: {}", e);
            return;
        }
    };

    let total_distance = calculate_total_distance(&left, &right);
    println!("Total distance: {}", total_distance);
}

pub fn part02() {
    let (left_str, right_str) = match read_input_file("inputs/day01.txt") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };

    let (left, right) = match cleaned_input(&left_str, &right_str) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error cleaning the data: {}", e);
            return;
        }
    };

    let similarity_score = calculate_similarity_score(&left, &right);
    println!("Similarity score: {}", similarity_score);
}

// Reads the input file and returns two vectors of type String
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

// Cleans the two vectors of input file by sorting and converting it to i32
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

// Converts the values in the array to i32
fn convert_to_i32(values: &[String]) -> Result<Vec<i32>, ParseIntError> {
    values.iter().map(|s| s.parse::<i32>()).collect()
}

// Calculates the total distance by summing up the difference of values on the same index of both
// arrays
fn calculate_total_distance(left: &[i32], right: &[i32]) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

// Creates a similarity score by:
// - Mapping the value counts
// - Counting the number repititions
// - Multiplyting the number of repitions to the value in left array
// - Summing it all up
fn calculate_similarity_score(left: &[i32], right: &[i32]) -> i32 {
    let mut count_map = HashMap::new();
    for &num in right {
        *count_map.entry(num).or_insert(0) += 1;
    }
    left.iter()
        .map(|&num| num * count_map.get(&num).unwrap_or(&0))
        .sum()
}
