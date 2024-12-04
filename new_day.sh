#!/bin/bash
DAY=$(printf "%02d" $1)
DIR="src/day${DAY}"

mkdir -p ${DIR}

cat <<EOT > ${DIR}/mod.rs
use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/day${DAY}/input.txt").expect("Failed to read input");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    0
}

fn part2(input: &str) -> i32 {
    0
}
EOT

cat <<EOT > ${DIR}/tests.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "test input data";
        assert_eq!(part1(input), 0); // Update expected result
    }

    #[test]
    fn test_part2() {
        let input = "test input data";
        assert_eq!(part2(input), 0); // Update expected result
    }
}
EOT

touch ${DIR}/input.txt

cat <<EOT > ${DIR}/notes.md
# Day ${DAY}: [Puzzle Title]

## Problem Description
- **Part 1**: (Summarize the problem here.)
- **Part 2**: (Summarize the problem here.)

---

## Brainstorming
- Input structure:
- Key observations:
- Edge cases:

---

## Plan
1. Parse the input.
2. Implement solution for part 1:
   - Steps:
3. Solve part 2 (if revealed):
   - Steps:
4. Write tests for edge cases and example cases.

---

## TODO
- [ ] Parse input.
- [ ] Write the \`part1\` function.
- [ ] Test \`part1\` with example data.
- [ ] Solve part 2.
- [ ] Optimize if needed.

---

## Notes
- (Additional thoughts about the problem.)

---

## Example Cases
| Input                   | Expected Output Part 1 | Expected Output Part 2 |
|--------------------------|------------------------|-------------------------|
|                          |                        |                         |
EOT

echo "Setup complete for day ${DAY}"

