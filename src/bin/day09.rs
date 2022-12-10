use std::collections::HashSet;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day09.txt")?;
    let start = std::time::Instant::now();

    let mut rope = [[0i32; 2]; 10];
    let mut part1_visited = HashSet::new();
    let mut part2_visited = HashSet::new();
    for line in input.lines() {
        let (dir, n) = line.split_once(' ').context("bad input")?;
        let delta = match dir.trim() {
            "U" => [0, 1],
            "D" => [0, -1],
            "L" => [-1, 0],
            "R" => [1, 0],
            _ => anyhow::bail!("unknown direction"),
        };

        for _ in 0..n.parse()? {
            rope[0] = std::array::from_fn(|i| rope[0][i] + delta[i]);
            for t in 1..10 {
                let (head, tail) = (rope[t - 1], &mut rope[t]);
                if (0..2).any(|axis| head[axis].abs_diff(tail[axis]) > 1) {
                    *tail = std::array::from_fn(|i| tail[i] + (head[i] - tail[i]).signum());
                } else {
                    break;
                }
            }
            part1_visited.insert(rope[1]);
            part2_visited.insert(rope[9]);
        }
    }

    println!("part1: {}", part1_visited.len());
    println!("part2: {}", part2_visited.len());
    println!("time: {:?}", start.elapsed());
    Ok(())
}
