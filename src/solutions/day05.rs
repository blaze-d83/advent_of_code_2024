use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let file_path = "inputs/day05.txt";
    match read_input_file(file_path) {
        Ok((rules, _updates)) => {
            let edges: Vec<[usize; 2]> = rules
                .into_iter()
                .map(|rule| {
                    if rule.len() != 2 {
                        panic!(
                            "Expected each rule to contain exactly 2 integers, got: {:?}",
                            rule
                        );
                    }
                    [rule[0] as usize, rule[1] as usize]
                })
                .collect();
            let size = edges
                .iter()
                .fold(0, |acc, edge| acc.max(edge[0]).max(edge[1]))
                + 1;

            match topological_sort(size, &edges) {
                Some(order) => {
                    println!(
                        "First five nodes in topological order: {:?}",
                        &order[..order.len().min(5)]
                    );
                }
                None => eprintln!("Cycle detected or invalid graph structure"),
            }
        }

        Err(e) => eprintln!("Error: {}", e),
    }
}

fn topological_sort(size: usize, edges: &Vec<[usize; 2]>) -> Option<Vec<usize>> {
    let mut graph = vec![Vec::new(); size];
    let mut indegree = vec![0; size];

    for edge in edges {
        let (from, to) = (edge[0], edge[1]);
        graph[from].push(to);
        indegree[to] += 1;
    }

    let mut queue = VecDeque::new();
    for node in 0..size {
        if indegree[node] == 0 {
            queue.push_back(node);
        }
    }

    let mut order = Vec::new();

    while let Some(node) = queue.pop_front() {
        order.push(node);

        for &neighbor in &graph[node] {
            indegree[neighbor] -= 1;
            if indegree[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    if order.len() == size {
        Some(order)
    } else {
        None
    }
}

fn test_read_input_file(rules: Vec<Vec<i32>>, updates: Vec<Vec<i32>>) {
    println!("TESTING RULES: ");
    for (i, rule) in rules.iter().take(5).enumerate() {
        println!("{}: {:?}", i + 1, rule);
    }
    println!("\nTESTING UPDATES: ");
    for (i, updates) in updates.iter().take(5).enumerate() {
        println!("{}: {:?}", i + 1, updates);
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
