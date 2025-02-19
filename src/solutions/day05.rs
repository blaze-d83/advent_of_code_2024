use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let file_path = "inputs/day05.txt";
    match read_input_file(file_path) {
        Ok((rules, updates)) => {
            let mut sum_middle = 0;
            for update in &updates {
                if is_update_valid(update, &rules) {
                    let mid_page = update[update.len() / 2];
                    sum_middle += mid_page;
                }
            }
            println!("Sum of middle page: {}", sum_middle);
        }

        Err(e) => eprintln!("Error: {}", e),
    }
}

fn is_update_valid(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    for rule in rules {
        if rule.len() < 2 {
            continue;
        }
        let u = rule[0];
        let v = rule[1];

        if let (Some(pos_u), Some(pos_v)) = (
            update.iter().position(|&x| x == u),
            update.iter().position(|&x| x == v),
        ) {
            if pos_u > pos_v {
                return false;
            }
        }
    }
    return true;
}

fn read_input_file(path: &str) -> Result<(Vec<Vec<i32>>, Vec<Vec<i32>>), std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut temp_rules: Vec<String> = Vec::new();
    let mut temp_updates: Vec<String> = Vec::new();

    let mut is_rules = true;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            is_rules = false;
            continue;
        }

        if is_rules {
            temp_rules.push(line);
        } else {
            temp_updates.push(line);
        }
    }

    let rules = temp_rules
        .iter()
        .map(|line| {
            line.split('|')
                .map(|num| num.trim().parse::<i32>().expect("Invalid integer in rules"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let updates = temp_updates
        .iter()
        .map(|line| {
            line.split(',')
                .map(|num| {
                    num.trim()
                        .parse::<i32>()
                        .expect("Invalid integer in updates")
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Ok((rules, updates))
}
