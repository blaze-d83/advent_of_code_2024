use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() {
    let file_path = "inputs/day08.txt";
    match read_input(file_path) {
        Ok(grid) => {
            let count = count_antinodes1(&grid);
            println!("Part 1 antinode count: {}", count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn part2() {
    let file_path = "inputs/day08.txt";
    match read_input(file_path) {
        Ok(grid) => {
            let count = count_antinodes2(&grid);
            println!("Part 2 antinode count: {}", count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn count_antinodes1(grid: &[Vec<char>]) -> u32 {
    if grid.is_empty() {
        return 0;
    }
    let h = grid.len();
    let w = grid[0].len();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let c = grid[y][x];
            if c != '.' {
                antennas.entry(c).or_default().push((x, y));
            }
        }
    }

    let mut antinodes = HashSet::new();
    for points in antennas.values() {
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                for &(cx, cy) in &[
                    (x2 as isize + dx, y2 as isize + dy),
                    (x1 as isize - dx, y1 as isize - dy),
                ] {
                    if (0..w as isize).contains(&cx) && (0..h as isize).contains(&cy) {
                        let (ux, uy) = (cx as usize, cy as usize);
                        antinodes.insert((ux, uy));
                    }
                }
            }
        }
    }

    antinodes.len() as u32
}

fn count_antinodes2(grid: &[Vec<char>]) -> u32 {
    if grid.is_empty() {
        return 0;
    }

    let h = grid.len();
    let w = grid[0].len();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let c = grid[y][x];
            if c != '.' {
                antennas.entry(c).or_default().push((x, y));
            }
        }
    }

    let mut anitnodes: HashSet<(usize, usize)> = HashSet::new();
    for points in antennas.values() {
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];

                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;
                let steps = gcd(dx.abs(), dy.abs());

                let step_x = dx / steps;
                let step_y = dy / steps;

                for k in 0..=steps {
                    let cx = x1 as isize + k * step_x;
                    let cy = y1 as isize + k * step_y;

                    if (0..w as isize).contains(&cx) && (0..h as isize).contains(&cy) {
                        anitnodes.insert((cx as usize, cy as usize));
                    }
                }
            }
        }
    }

    anitnodes.len() as u32
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

fn read_input(path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .map(|l| l.map(|s| s.chars().collect()))
        .collect::<Result<_, _>>()?;
    Ok(grid)
}

// that worked. Can you give me a psuedo-code to understand this solution intuitively along with few takeaways to help me use some of the techniques employed here in future?
