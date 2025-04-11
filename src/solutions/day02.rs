use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let data = match read_input_file("inputs/day02.txt") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading the input file: {}", e);
            return;
        }
    };

    let safe_count = data.iter().filter(|row| is_safe(row)).count();

    println!("Safe Reports: {}", safe_count);
}

pub fn part02() {
    let data = match read_input_file("inputs/day02.txt") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading the input file: {}", e);
            return;
        }
    };

    let safe_count = data.iter().filter(|row| problem_dampner(row)).count();

    println!("Safe count with Problem Dampner: {}", safe_count);
}

// Tries to remove one element from the row to see if it becomes safe
fn problem_dampner(row: &Vec<i32>) -> bool {
    if is_safe(row) {
        return true;
    }

    for i in 0..row.len() {
        let mut modified_row = row.clone();
        modified_row.remove(i);

        if is_safe(&modified_row) {
            return true;
        }
    }
    false
}

// Checks if the row is safe based on specific conditions
// - Fewwer than 2 elements
// - Is either ascending / descending order
// - The difference between the next values is not greater than 3
fn is_safe(row: &Vec<i32>) -> bool {
    if row.len() < 2 {
        return true;
    }

    let mut ascending = true;
    let mut descending = true;

    for window in row.windows(2) {
        let diff = (window[0] - window[1]).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        if window[0] > window[1] {
            ascending = false;
        }

        if window[0] < window[1] {
            descending = false;
        }
    }

    ascending || descending
}

// Reads the input file and retunrs a vector or integer vectors
fn read_input_file(path: &str) -> Result<Vec<Vec<i32>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let values: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        data.push(values);
    }
    Ok(data)
}
