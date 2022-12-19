use anyhow::{Ok, Result};
use aoc2022::{RegexExtract, OptionSomeExt};
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day04.txt")?;
    let start = std::time::Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)")?;
    for line in input.lines() {
        let assignment = re.extract(line).some()?.1;
        let [s1, e1, s2, e2] = assignment.map(|id| id.parse::<u64>().unwrap());
        part1 += (s1 <= s2 && e2 <= e1 || s2 <= s1 && e1 <= e2) as u64;
        part2 += (s1 <= e2 && s2 <= e1) as u64;
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
