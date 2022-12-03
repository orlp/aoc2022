use anyhow::{Ok, Result};
use itertools::Itertools;

fn rucksack_bitset(rucksack: &str) -> u64 {
    let priorities = rucksack.bytes().map(|item| match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("unknown item"),
    });
    priorities.map(|p| 1u64 << p).fold(0, |a, b| a | b)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day03.txt")?;
    let start = std::time::Instant::now();
    let part1_common = input.lines().map(|line| {
        let (a, b) = line.split_at(line.len() / 2);
        (rucksack_bitset(a) & rucksack_bitset(b)).trailing_zeros()
    });
    let part2_groups = input.lines().map(rucksack_bitset).tuples();
    let part2_common = part2_groups.map(|(a, b, c)| (a & b & c).trailing_zeros());
    println!("time: {:?}", start.elapsed());
    println!("part1: {}", part1_common.sum::<u32>());
    println!("part2: {}", part2_common.sum::<u32>());
    Ok(())
}
