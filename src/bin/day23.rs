use anyhow::{bail, Ok, Result};
use hashbrown::{HashMap, HashSet};
use itertools::{Itertools, MinMaxResult};
use MinMaxResult::MinMax;

fn round(elves: &mut HashSet<(i64, i64)>, first_dir: usize) -> bool {
    let mut proposals: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    let mut new_elves = HashSet::with_capacity(elves.len());
    'next_elf: for elf in elves.iter() {
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let valid = dirs.map(|(dx, dy)| {
            !elves.contains(&(elf.0 + dx - dy, elf.1 + dy - dx))
                && !elves.contains(&(elf.0 + dx, elf.1 + dy))
                && !elves.contains(&(elf.0 + dx + dy, elf.1 + dy + dx))
        });
        if !valid.iter().all(|x| *x) {
            for dir in (0..4).map(|i| (first_dir + i) % 4) {
                if valid[dir] {
                    let dest = (elf.0 + dirs[dir].0, elf.1 + dirs[dir].1);
                    proposals.entry(dest).or_default().push(*elf);
                    continue 'next_elf;
                }
            }
        }
        new_elves.insert(*elf);
    }

    let mut moved = false;
    for (dest, candidates) in proposals {
        match &candidates[..] {
            &[_elf] => {
                drop(new_elves.insert(dest));
                moved = true;
            },
            elves => new_elves.extend(elves),
        }
    }
    *elves = new_elves;
    moved
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day23.txt")?;
    let start = std::time::Instant::now();

    let mut elves = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, symbol) in line.bytes().enumerate() {
            if symbol == b'#' {
                elves.insert((col as i64, row as i64));
            }
        }
    }

    for r in 0..10 {
        round(&mut elves, r);
    }
    let MinMax(lox, hix) = elves.iter().map(|(x, _y)| x).minmax() else { bail!("no elves") };
    let MinMax(loy, hiy) = elves.iter().map(|(_x, y)| y).minmax() else { bail!("no elves") };
    let part1 = (hix + 1 - lox) * (hiy + 1 - loy) - elves.len() as i64;
    let part2 = 11 + (10..).take_while(|r| round(&mut elves, *r)).count();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
