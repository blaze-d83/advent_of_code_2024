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
        Err(e) => eprint!("Error: {}", e),
    }
}

#[derive(Debug, Clone, Copy)]
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
