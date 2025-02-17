use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let file_path = "inputs/day05.txt";
    match read_input_file(file_path) {
        Ok((rules, updates)) => {
            println!("TESTING RULES: ");
            for (i, rule) in rules.iter().take(5).enumerate() {
                println!("{}: {:?}", i + 1, rule);
            }
            println!("\n TESTING UPDATES: ");
            for (i, updates) in updates.iter().take(5).enumerate() {
                println!("{}: {:?}", i + 1, updates);
            }
        }

        Err(e) => eprintln!("Error: {}", e),
    }
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
