use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn part1() {
    let file_path = "inputs/day07.txt";
    match read_input_file(file_path) {
        Ok(data) => {
            let mut total_sum = 0;
            for (target, nums) in data {
                if can_make_target(&nums, target) {
                    total_sum += target;
                }
            }
            println!("Calibration result: {}", total_sum);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[derive(Debug, Clone, Copy)]
enum Operators {
    Add,
    Multiply,
}

fn apply_op(a: i64, b: i64, op: Operators) -> Option<i64> {
    match op {
        Operators::Add => Some(a + b),
        Operators::Multiply => Some(a * b),
    }
}

fn can_make_target(nums: &[i64], target: i64) -> bool {
    fn recurse(index: usize, current: i64, nums: &[i64], target: i64) -> bool {
        if index == nums.len() {
            return current == target;
        }
        let next = nums[index];
        for op in [Operators::Add, Operators::Multiply].iter() {
            if let Some(result) = apply_op(current, next, *op) {
                if recurse(index + 1, result, nums, target) {
                    return true;
                }
            }
        }
        false
    }
    if nums.is_empty() {
        return false;
    }
    recurse(1, nums[0], nums, target)
}

pub fn read_input_file(path: &str) -> Result<Vec<(i64, Vec<i64>)>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        let target = parts[0]
            .trim()
            .parse::<i64>()
            .expect("failed to parse target number");

        let values: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>().expect("failed to parse the value"))
            .collect();

        data.push((target, values));
    }
    Ok(data)
}
