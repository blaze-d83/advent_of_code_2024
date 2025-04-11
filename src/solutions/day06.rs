use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let file_path = "inputs/day06.txt";
    match read_input_file(file_path) {
        Ok(map) => {
            let count = simulate(&map);
            println!("Steps: {}", count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn part02() {
    let file_path = "inputs/day06.txt";
    match read_input_file(file_path) {
        Ok(map) => {
            let count = count_loop_positions(&map);
            println!("Loop Count: {}", count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }
}

fn find_starting_pos(map: &[Vec<char>]) -> Option<((usize, usize), Direction)> {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let direction = match cell {
                '>' => Some(Direction::Right),
                'v' => Some(Direction::Down),
                '<' => Some(Direction::Left),
                '^' => Some(Direction::Up),
                _ => None,
            };
            if let Some(dir) = direction {
                return Some(((x, y), dir));
            }
        }
    }
    None
}

/// Simulates the guard’s movement on the given map (with no added obstruction)
/// and counts steps only on first visits.
fn simulate(map: &[Vec<char>]) -> usize {
    let (mut pos, mut dir) = find_starting_pos(map).expect("No starting direction found");
    let rows = map.len();
    let cols = map[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(pos);

    let mut steps = 1;

    loop {
        let (dx, dy) = dir.delta();
        let next_x = pos.0 as i32 + dx;
        let next_y = pos.1 as i32 + dy;

        if next_x < 0 || next_x >= cols as i32 || next_y < 0 || next_y >= rows as i32 {
            break;
        }

        let next_pos = (next_x as usize, next_y as usize);

        if map[next_pos.1][next_pos.0] == '#' {
            dir = dir.turn_right();
            continue;
        }

        pos = next_pos;

        if visited.insert(pos) {
            steps += 1;
        }
    }

    steps
}

/// Simulate the guard’s path with a candidate cell treated as an obstacle.
/// Returns true if the guard eventually loops.
fn simulate_with_obstacle(map: &[Vec<char>], candidate: (usize, usize)) -> bool {
    // Obtain starting state.
    let (start_pos, start_dir) = find_starting_pos(map).expect("No starting position found");
    let rows = map.len();
    let cols = map[0].len();

    // Use a HashSet to track (position, direction) states.
    let mut seen_states: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;

    // Set an iteration limit to avoid infinite loops in pathological cases.
    let max_iterations = rows * cols * 4;
    for _ in 0..max_iterations {
        if !seen_states.insert((pos, dir)) {
            // State repeated → loop detected.
            return true;
        }

        let (dx, dy) = dir.delta();
        let next_x = pos.0 as i32 + dx;
        let next_y = pos.1 as i32 + dy;

        if next_x < 0 || next_x >= cols as i32 || next_y < 0 || next_y >= rows as i32 {
            // Guard escapes the map → no loop.
            return false;
        }

        let next_pos = (next_x as usize, next_y as usize);

        // If the next position is our candidate obstacle, treat it as blocked.
        if next_pos == candidate || map[next_pos.1][next_pos.0] == '#' {
            // Turn right and continue from the same position.
            dir = dir.turn_right();
            continue;
        }

        pos = next_pos;
    }

    // If we reached the iteration limit, assume no loop.
    false
}

/// Iterates over every valid candidate position and counts those that would cause a loop.
fn count_loop_positions(map: &[Vec<char>]) -> usize {
    let (start_pos, _) = find_starting_pos(map).expect("No starting position found");
    let mut count = 0;
    let rows = map.len();
    let cols = map[0].len();

    for y in 0..rows {
        for x in 0..cols {
            // Skip positions that already contain an obstacle.
            if map[y][x] == '#' {
                continue;
            }
            // The candidate cannot be the starting position.
            if (x, y) == start_pos {
                continue;
            }
            if simulate_with_obstacle(map, (x, y)) {
                count += 1;
            }
        }
    }
    count
}

fn read_input_file(path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let map: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    Ok(map)
}
