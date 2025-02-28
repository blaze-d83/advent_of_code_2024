use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Operators {
    Add,
    Multiply,
}

impl Operators {}

/// Reads the input file "input.txt" and returns a HashMap that maps
/// each key (u32) to a vector of u32 values.
///
/// Each line in the file should have the format:
///     key: value1 value2 ...
pub fn read_input_file() -> Result<HashMap<u32, Vec<u32>>, io::Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    parse_input(reader)
}

/// Parses input from any type that implements BufRead.
/// This function is generic so it can be reused in tests or for other input sources.
fn parse_input<R: BufRead>(reader: R) -> Result<HashMap<u32, Vec<u32>>, io::Error> {
    let mut map = HashMap::new();

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Use splitn to ensure we only split into two parts: key and values.
        let mut parts = line.splitn(2, ':');
        let key_str = parts.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Line {}: Missing key in '{}'", line_number + 1, line),
            )
        })?;
        let values_str = parts.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Line {}: Missing values in '{}'", line_number + 1, line),
            )
        })?;

        let key = key_str.trim().parse::<u32>().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Line {}: Error parsing key '{}': {}",
                    line_number + 1,
                    key_str.trim(),
                    e
                ),
            )
        })?;

        let values = values_str
            .trim()
            .split_whitespace()
            .map(|v| {
                v.parse::<u32>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Line {}: Error parsing value '{}': {}",
                            line_number + 1,
                            v,
                            e
                        ),
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        map.insert(key, values);
    }
    Ok(map)
}
