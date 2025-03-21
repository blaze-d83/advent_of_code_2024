use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

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
