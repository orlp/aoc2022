use anyhow::{Context, Result};
use hashbrown::HashSet;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day09.txt")?;
    let start = std::time::Instant::now();

    let mut rope = [(0i32, 0i32); 10];
    let mut part1_visited: HashSet<_> = HashSet::with_capacity(2048);
    let mut part2_visited: HashSet<_> = HashSet::with_capacity(2048);
    for line in input.lines() {
        let (dir, n) = line.split_once(' ').context("bad input")?;
        let delta = match dir.trim() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => anyhow::bail!("unknown direction"),
        };

        for _ in 0..n.parse()? {
            rope[0].0 += delta.0;
            rope[0].1 += delta.1;
            for t in 1..10 {
                let (head, tail) = (rope[t - 1], &mut rope[t]);
                if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                    tail.0 += (head.0 - tail.0).signum();
                    tail.1 += (head.1 - tail.1).signum();
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
