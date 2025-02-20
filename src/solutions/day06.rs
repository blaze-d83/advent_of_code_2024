use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part01() {
    let file_path = "inputs/day06.txt";
    match read_input_file(file_path) {
        Ok(map) => {
            for line in map.iter().take(5) {
                let line_str: String = line.iter().collect();
                println!("{}", line_str);
            }
        }
        Err(e) => eprint!("Error: {}", e),
    }
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
