use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let file_path = "inputs/day04.txt";
    match read_input_file(file_path) {
        Ok(data) => {
            let result = find_xmas(data);
            println!("Count: {}", result);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn part02() {
    let file_path = "inputs/day04.txt";
    match read_input_file(file_path) {
        Ok(data) => {
            let result = find_mas(data);
            println!("Count: {}", result);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

/// Scans the provided grid for occurrences of the sequence "XMAS" in all eight directions.
///
/// The search is performed by iterating over each cell in the grid and checking every
/// contiguous sequence of four characters along the eight possible directions:
/// right, down-right, down, down-left, left, up-left, up, and up-right.
///
/// # Arguments
///
/// * `grid` - A two-dimensional vector of characters representing the input grid.
///
/// # Returns
///
/// * An integer count representing the number of times the sequence `['X', 'M', 'A', 'S']`
///   appears in the grid.
///
/// # Panics
///
/// This function will panic if the grid is empty or if rows have inconsistent lengths.
///
fn find_xmas(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),   // right
        (1, 1),   // down-right
        (1, 0),   // down
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, -1), // up-left
        (-1, 0),  // up
        (-1, 1),  // up-right
    ];

    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &DIRECTIONS {
                let mut valid = true;

                // Check all four positions
                for step in 0..4 {
                    let ni = i as i32 + dx * step as i32;
                    let nj = j as i32 + dy * step as i32;
                    if ni < 0 || ni >= rows as i32 || nj < 0 || nj >= cols as i32 {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    let chars: Vec<char> = (0..4)
                        .map(|step| {
                            let ni = i as i32 + dx * step as i32;
                            let nj = j as i32 + dy * step as i32;
                            grid[ni as usize][nj as usize]
                        })
                        .collect();
                    if chars == ['X', 'M', 'A', 'S'] {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

/// Counts X‑MAS patterns in a grid.
///
/// An X‑MAS pattern has an 'A' at its center and two diagonal pairs that are either
/// ('M', 'S') or ('S', 'M') (order doesn't matter). Only cells with all four diagonal
/// neighbors (i.e. not on the border) are checked.
///
/// # Arguments
///
/// * `grid` - A rectangular Vec<Vec<char>> representing the grid.
///
/// # Returns
///
/// The number of valid X‑MAS patterns found.
fn find_mas(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if grid[i][j] != 'A' {
                continue;
            }

            let d1 = (grid[i - 1][j - 1], grid[i + 1][j + 1]);
            let d2 = (grid[i - 1][j + 1], grid[i + 1][j - 1]);

            let valid_diag =
                |(c1, c2): (char, char)| (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M');
            if valid_diag(d1) && valid_diag(d2) {
                count += 1;
            }
        }
    }

    count
}

// Read the input file from the file path and returns a 2D matrix of chars
fn read_input_file(path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    Ok(grid)
}
