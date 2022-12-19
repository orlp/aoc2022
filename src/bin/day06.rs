use anyhow::{Context, Ok, Result};

fn rolling_distinct_windows(s: &[u8], n: usize) -> impl Iterator<Item = (&[u8], usize)> {
    let mut in_window = [0; 256];
    let mut count = 0;
    (0..s.len()).filter_map(move |i| {
        if i >= n {
            in_window[s[i - n] as usize] -= 1;
            count -= (in_window[s[i - n] as usize] == 0) as usize;
        }
        in_window[s[i] as usize] += 1;
        count += (in_window[s[i] as usize] == 1) as usize;
        (i >= n - 1).then(|| (&s[i + 1 - n..i + 1], count))
    })
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day06.txt")?;
    let start = std::time::Instant::now();

    let bytes = input.trim().as_bytes();
    let p1 = rolling_distinct_windows(bytes, 4).position(|(_, c)| c == 4);
    let p2 = rolling_distinct_windows(bytes, 14).position(|(_, c)| c == 14);
    println!("part1: {}", p1.context("marker not found")? + 4);
    println!("part2: {}", p2.context("marker not found")? + 14);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
