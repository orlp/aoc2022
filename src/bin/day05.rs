use anyhow::{Ok, Result};
use aoc2022::{GetDisjointMut, OptionSomeExt, RegexExtract};
use itertools::Itertools;
use regex::Regex;

fn stack_heads(mut stacks: Vec<Vec<u8>>, instructions: &[&str], reverse: bool) -> Result<String> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    for line in instructions {
        let instr = re.extract(line).some()?.1;
        let [len, from, to] = instr.map(|x| x.parse::<usize>().unwrap());
        let locs = [from.wrapping_sub(1), to.wrapping_sub(1)];
        let [from_stack, to_stack] = stacks.get_disjoint_mut(locs).some()?;
        if reverse {
            to_stack.extend(from_stack.drain(from_stack.len() - len..).rev());
        } else {
            to_stack.extend(from_stack.drain(from_stack.len() - len..));
        }
    }

    let stack_heads = stacks.iter().flat_map(|s| s.last());
    Ok(stack_heads.map(|c| *c as char).collect())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day05.txt")?;
    let start = std::time::Instant::now();

    let nonempty = input.lines().map(str::trim).filter(|l| !l.is_empty());
    let lines = nonempty.collect_vec();
    let labels_line = lines.iter().position(|l| l.starts_with('1')).some()?;
    let stacks = (0..(lines[labels_line].len() + 3) / 4).map(|s| {
        let rows = lines[..labels_line].iter().rev();
        let stack_items = rows.filter(|r| r.as_bytes().get(4 * s) == Some(&b'['));
        stack_items.map(|r| r.as_bytes()[4 * s + 1]).collect()
    });

    let part1 = stack_heads(stacks.clone().collect(), &lines[labels_line + 1..], true)?;
    let part2 = stack_heads(stacks.collect(), &lines[labels_line + 1..], false)?;

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
