use anyhow::{Context, Ok, Result};

fn find_disjoint_window(s: &[u8], n: usize) -> Option<usize> {
    let mut last_known_position = [0; 256];
    let mut start_disjoint = 0;
    for i in 0..s.len() {
        start_disjoint = start_disjoint.max(last_known_position[s[i] as usize] + 1);
        last_known_position[s[i] as usize] = i;
        if i >= start_disjoint + n {
            return Some(i);
        }
    }
    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day06.txt")?;
    let start = std::time::Instant::now();

    let bytes = input.trim().as_bytes();
    let p1 = find_disjoint_window(bytes, 4);
    let p2 = find_disjoint_window(bytes, 14);
    println!("part1: {}", p1.context("marker not found")?);
    println!("part2: {}", p2.context("marker not found")?);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
