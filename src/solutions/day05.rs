use std::{
    collections::{HashMap, HashSet, VecDeque},
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

pub fn part02() {
    let file_path = "inputs/day05.txt";
    match read_input_file(file_path) {
        Ok((rules, updates)) => {
            let mut sum_middle = 0;
            for update in &updates {
                if !is_update_valid(update, &rules) {
                    let correct_order = reorder_updates(update, &rules);
                    let mid_page = correct_order[correct_order.len() / 2];
                    sum_middle += mid_page;
                }
            }
            println!("Sumf of middle pages of corrected updates: {}", sum_middle);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Reorders an update based on provided ordering rules using topological sorting.
///
/// # Arguments
///
/// * `update` - A vector of page numbers representing an update.
/// * `rules` - A vector of rules, where each rule is a two-element vector `[u, v]`
///   indicating that page `u` must come before page `v`.
///
/// # Returns
///
/// A vector of page numbers representing the update reordered to satisfy the ordering rules.
///
/// # Details
///
/// The function constructs a directed graph where nodes are the pages present in the update and
/// edges are derived from the rules. It then computes the in-degrees of each node and performs
/// a topological sort (using Kahn's algorithm) to determine an order that respects the dependencies.
fn reorder_updates(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut nodes: HashSet<i32> = HashSet::new();

    // Add all pages from the updates to the graph
    for &page in update {
        graph.entry(page).or_insert(Vec::new());
        nodes.insert(page);
    }

    // Process valid rules
    for rule in rules {
        if rule.len() != 2 {
            continue;
        }

        let u = rule[0];
        let v = rule[1];
        if nodes.contains(&u) && nodes.contains(&v) {
            graph.entry(u).or_default().push(v);
        }
    }

    // Compute in-degree for each node
    let mut in_degree: HashMap<i32, usize> = nodes.iter().map(|&node| (node, 0)).collect();

    for neighbors in graph.values() {
        for &nbr in neighbors {
            *in_degree.get_mut(&nbr).unwrap() += 1;
        }
    }

    // Intialize queue with nodes having 0 in-degree
    let mut queue = VecDeque::new();
    for (&node, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(node);
        }
    }

    // Perform topological sorting
    let mut sorted = Vec::new();
    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &nbr in neighbors {
                let entry = in_degree.get_mut(&nbr).unwrap();
                *entry -= 1;
                if *entry == 0 {
                    queue.push_back(nbr);
                }
            }
        }
    }
    sorted
}

/// Checks whether an update follows the ordering rules.
///
/// # Arguments
///
/// * `update` - A vector of page numbers representing an update.
/// * `rules` - A vector of rules, where each rule is a two-element vector `[u, v]` indicating
///   that page `u` must come before page `v`.
///
/// # Returns
///
/// `true` if the update respects all the rules (i.e. for every rule, if both pages are present,
/// the page `u` appears before page `v`); otherwise, returns `false`.
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

/// Reads the input file and parses the ordering rules and updates.
///
/// # Arguments
///
/// * `path` - The file path to the input file.
///
/// # Returns
///
/// A `Result` containing a tuple:
/// - The first element is a vector of rules (each rule is a vector of integers, with values
///   separated by a pipe `|` in the file).
/// - The second element is a vector of updates (each update is a vector of integers, with values
///   separated by commas `,` in the file).
///
/// # Errors
///
/// Returns an `std::io::Error` if the file cannot be opened or read.
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
