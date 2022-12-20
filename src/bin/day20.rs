use anyhow::{Ok, Result};
use aoc2022::OptionSomeExt;
use aoc2022::treap::Treap;
use itertools::Itertools;

fn decrypt(nums: &[i64], mult: i64, k: usize) -> Result<i64> {
    let zero_idx = nums.iter().position(|x| *x == 0).some()?;
    let mut rng = rand::thread_rng();
    let mut trp = Treap::default();
    let mut nodes = nums
        .iter()
        .enumerate()
        .map(|(i, n)| trp.insert(*n * mult, i, &mut rng))
        .collect_vec();

    for _ in 0..k {
        for node in &mut nodes {
            let (value, rank) = trp.remove(*node).some()?;
            let new_rank = (value + rank as i64).rem_euclid(nums.len() as i64 - 1);
            *node = trp.insert(value, new_rank as usize, &mut rng);
        }
    }

    let zero_rank = trp.rank(nodes[zero_idx]).some()?;
    let grove = (1..=3).map(|k| {
        let rank = trp.derank((zero_rank + 1000 * k) % nums.len());
        trp.get(rank).some()
    });
    Ok(itertools::process_results(grove, |it| it.sum())?)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day20.txt")?;
    let start = std::time::Instant::now();
    let nums: Vec<i64> = input.lines().map(|n| n.parse()).try_collect()?;
    println!("part1: {}", decrypt(&nums, 1, 1)?);
    println!("part2: {}", decrypt(&nums, 811589153, 10)?);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
