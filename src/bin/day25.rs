use anyhow::{Ok, Result};
use aoc2022::OptionSomeExt;

const SNAFU_DIGITS: [u8; 5] = [b'=', b'-', b'0', b'1', b'2'];

fn from_snafu(num: &str) -> Result<i64> {
    let (mut ret, mut pow) = (0, 1);
    for b in num.as_bytes().iter().rev() {
        let d = SNAFU_DIGITS.iter().position(|d| d == b).some()?;
        ret += (d as i64 - 2) * pow;
        pow *= 5;
    }
    Ok(ret)
}

fn to_snafu(mut num: i64) -> String {
    let mut ret = Vec::new();
    while num != 0 {
        let rem = (2 + num).rem_euclid(5);
        ret.push(SNAFU_DIGITS[rem as usize]);
        num = (num - (rem - 2)) / 5;
    }
    ret.reverse();
    unsafe { String::from_utf8_unchecked(ret) }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day25.txt")?;
    let start = std::time::Instant::now();
    
    let nums = input.lines().map(from_snafu);
    let part1 = to_snafu(itertools::process_results(nums, |it| it.sum())?);

    println!("part1: {part1}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
