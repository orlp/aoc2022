use anyhow::Result;
use itertools::Itertools;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Packet {
    List(Vec<Packet>),
    Int(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(x), Packet::List(y)) => x.cmp(y),
            (Packet::List(x), Packet::Int(y)) => x.cmp(&vec![Packet::Int(*y)]),
            (Packet::Int(x), Packet::List(y)) => vec![Packet::Int(*x)].cmp(&y),
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day13.txt")?;
    let start = std::time::Instant::now();

    let packets: Vec<Packet> = input.lines().filter(|s| s.trim().len() > 0).map(serde_json::from_str).try_collect()?;
    let correct_positions = packets.iter().tuples().positions(|(a, b)| a < b);
    let tworank = packets.iter().filter(|p| **p < Packet::Int(2)).count() + 1;
    let sixrank = packets.iter().filter(|p| **p < Packet::Int(6)).count() + 2;

    println!("part1: {}", correct_positions.map(|i| i + 1).sum::<usize>());
    println!("part2: {}", tworank * sixrank);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
