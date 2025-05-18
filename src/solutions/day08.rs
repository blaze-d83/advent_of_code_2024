use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() {
    let file_path = "inputs/day08.txt";
    match read_input(file_path) {
        Ok(grid) => {
            let count = count_antinotes(&grid);
            println!("Count: {}", count);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn count_antinotes(grid: &[Vec<char>]) -> u32 {
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

    let mut antinotes: HashSet<(usize, usize)> = HashSet::new();
    for points in antennas.values() {
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                let x3 = x2 as isize + dx;
                let y3 = y2 as isize + dy;
                if (0..w as isize).contains(&x3) && (0..h as isize).contains(&y3) {
                    if grid[y3 as usize][x3 as usize] == '.' {
                        antinotes.insert((x3 as usize, y3 as usize));
                    }
                }

                let x0 = x1 as isize - dx;
                let y0 = y1 as isize - dy;
                if (0..w as isize).contains(&x0) && (0..h as isize).contains(&y0) {
                    if grid[y0 as usize][x0 as usize] == '.' {
                        antinotes.insert((x0 as usize, y0 as usize));
                    }
                }
            }
        }
    }

    antinotes.len() as u32
}

fn read_input(path: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }
    Ok(grid)
}
