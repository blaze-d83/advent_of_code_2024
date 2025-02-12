use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let data = match read_input_file("inputs/day02.txt") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut safe_count = 0;
    for row in &data {
        if is_safe(row) {
            safe_count += 1;
        }
    }
    println!("Safe Reports: {}", safe_count);
}

fn is_safe(row: &Vec<i32>) -> bool {
    if row.len() < 2 {
        return true;
    }

    let mut ascending = true;
    let mut descending = true;

    for i in 0..row.len() - 1 {
        let diff = (row[i] - row[i + 1]).abs();
        if diff > 3 || diff < 1 {
            return false;
        }
        if row[i] > row[i + 1] {
            ascending = false;
        }
        if row[i] < row[i + 1] {
            descending = false;
        }
    }
    ascending || descending
}

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
