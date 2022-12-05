use anyhow::{Context, Ok, Result};
use aoc2022::{GetDisjointMut, RegexExtract};
use regex::Regex;

fn stack_heads(mut stacks: Vec<Vec<u8>>, instructions: &[&str], reverse: bool) -> Result<String> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;
    for line in instructions {
        let instr = re.extract(line).context("cannot parse move instr")?.1;
        let [len, from, to] = instr.map(|x| x.parse::<usize>().unwrap());
        let locs = [from.wrapping_sub(1), to.wrapping_sub(1)];
        let [from_stack, to_stack] = stacks.get_disjoint_mut(locs).context("invalid instr")?;
        if reverse {
            to_stack.extend(from_stack.drain(from_stack.len() - len..).rev());
        } else {
            to_stack.extend(from_stack.drain(from_stack.len() - len..));
        }
    }

    Ok(stacks.iter().flat_map(|s| s.last()).map(|c| *c as char).collect())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day05.txt")?;
    let start = std::time::Instant::now();

    let lines: Vec<_> = input.lines().filter(|l| l.trim().len() > 0).collect();
    let labels_line = lines.iter().position(|l| l.trim().starts_with('1')).context("no labels")?;
    let stacks = (0..(lines[labels_line].len() + 3) / 4).map(|s| {
        let rows = lines[..labels_line].iter().rev();
        let stack_items = rows.filter(|r| r.as_bytes().get(4 * s) == Some(&b'['));
        stack_items.map(|r| r.as_bytes()[4 * s + 1]).collect()
    });

    println!("part1: {}", stack_heads(stacks.clone().collect(), &lines[labels_line + 1..], true)?);
    println!("part2: {}", stack_heads(stacks.collect(), &lines[labels_line + 1..], false)?);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
