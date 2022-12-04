use anyhow::{Ok, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day01.txt")?;
    let start = std::time::Instant::now();

    let groups = input.lines().map(|l| l.trim()).group_by(|l| l.len() > 0);
    let nonempty_groups = groups.into_iter().filter_map(|(b, g)| b.then_some(g));
    let mut sums: Vec<i64> = nonempty_groups
        .map(|g| g.map(|l| l.parse::<i64>()).fold_ok(0, |a, b| a + b))
        .try_collect()?;
    sums.select_nth_unstable_by_key(2, |s| std::cmp::Reverse(*s));

    println!("time: {:?}", start.elapsed());
    println!("part1: {}", sums[..3].iter().copied().max().unwrap_or(0));
    println!("part2: {}", sums[..3].iter().copied().sum::<i64>());
    Ok(())
}
