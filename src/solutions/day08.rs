use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn part1() {
    let file_path = "inputs/day08.txt";
    match read_input_file(file_path) {
        Ok(data) => {
            let rows = data.len();
            let cols = data[0].len();
            let map = map_frequencies(data);

            let total = total_antinodes(map, rows, cols);
            println!("Total Antinodes: {}", total);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn total_antinodes(map: HashMap<char, Vec<(usize, usize)>>, rows: usize, cols: usize) -> usize {
    let mut unique_nodes: HashSet<(usize, usize)> = HashSet::new();
    for (_key, positions) in map {
        unique_nodes.extend(count_extension_nodes(&positions, rows, cols));
    }
    unique_nodes.len()
}

fn check_in_bounds(row: isize, col: isize, height: usize, width: usize) -> bool {
    row >= 0 && row < height as isize && col >= 0 && col < width as isize
}

fn extension_nodes_for_pair(
    p1: (usize, usize),
    p2: (usize, usize),
    grid_height: usize,
    grid_width: usize,
) -> Vec<(usize, usize)> {
    let dr = p2.0 as isize - p1.0 as isize;
    let dc = p2.1 as isize - p1.1 as isize;
    let mut nodes = Vec::new();

    let (mut forward_r, mut forward_c) = (p2.0 as isize, p2.1 as isize);
    let (mut backward_r, mut backward_c) = (p1.0 as isize, p1.1 as isize);

    loop {
        let mut extended = false;

        forward_r += dr;
        forward_c += dc;
        if check_in_bounds(forward_r, forward_c, grid_height, grid_width) {
            nodes.push((forward_r as usize, forward_c as usize));
            extended = true;
        }

        backward_r -= dr;
        backward_c -= dc;
        if check_in_bounds(backward_r, backward_c, grid_height, grid_width) {
            nodes.push((backward_r as usize, backward_c as usize));
            extended = true;
        }

        if !extended {
            break;
        }
    }

    nodes
}

fn count_extension_nodes(
    positions: &[(usize, usize)],
    grid_height: usize,
    grid_width: usize,
) -> HashSet<(usize, usize)> {
    let mut nodes_set: HashSet<(usize, usize)> = HashSet::new();

    for window in positions.windows(2) {
        let p1 = window[0];
        let p2 = window[1];
        let nodes = extension_nodes_for_pair(p1, p2, grid_height, grid_width);
        for node in nodes {
            nodes_set.insert(node);
        }
    }

    nodes_set
}

fn map_frequencies(input: Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if !ch.is_ascii_punctuation() {
                map.entry(ch)
                    .or_insert_with(Vec::new)
                    .push((row_idx, col_idx));
            }
        }
    }
    map
}

fn read_input_file(path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.map(|l| l.chars().collect()))
        .collect::<Result<Vec<Vec<char>>, io::Error>>()?;

    Ok(grid)
}
