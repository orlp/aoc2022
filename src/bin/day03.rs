use anyhow::{Ok, Result};
use itertools::Itertools;

fn rucksack_bitset(rucksack: &str) -> u64 {
    rucksack
        .bytes()
        .map(|item| match item {
            b'a'..=b'z' => 1u64 << (item - b'a' + 1),
            b'A'..=b'Z' => 1u64 << (item - b'A' + 27),
            _ => panic!("unknown item"),
        })
        .fold(0, |a, b| a | b)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day03.txt")?;
    let start = std::time::Instant::now();
    let part1_common = input.lines().map(|line| {
        let (a, b) = line.split_at(line.len() / 2);
        let [ap, bp] = [a, b].map(rucksack_bitset);
        (ap & bp).trailing_zeros()
    });
    let part2_common = input.lines().tuples().map(|(a, b, c)| {
        let [ap, bp, cp] = [a, b, c].map(rucksack_bitset);
        (ap & bp & cp).trailing_zeros()
    });
    println!("time: {:?}", start.elapsed());
    println!("part1: {}", part1_common.sum::<u32>());
    println!("part2: {}", part2_common.sum::<u32>());
    Ok(())
}
