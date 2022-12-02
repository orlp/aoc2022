use anyhow::{Context, Ok, Result};
use aoc2022::RegexExtract;
use regex::Regex;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day02.txt")?;
    let start = std::time::Instant::now();
    let re = Regex::new("([ABC]) ([XYZ])")?;
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let [abc, xyz] = re.extract(line).context("invalid strategy")?.1;
        let [abc, xyz] = [abc.as_bytes()[0] - b'A', xyz.as_bytes()[0] - b'X'];

        // 0 = Rock, 1 = Paper, 2 = Scissor, (k + 1) mod 3 thus defeats k.
        // 0 = Defeat, 1 = Draw, 2 = Victory
        // Identity: 1 + ours - theirs = outcome   (mod 3)
        let p1_outcome = (1 + xyz + (3 - abc)) % 3;
        let p2_shape = (xyz + abc + (3 - 1)) % 3;
        part1 += (1 + xyz + 3 * p1_outcome) as u64;
        part2 += (1 + p2_shape + 3 * xyz) as u64;
    }
    println!("time: {:?}", start.elapsed());
    println!("part1: {part1}");
    println!("part2: {part2}");
    Ok(())
}
