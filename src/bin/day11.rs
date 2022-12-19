use std::collections::VecDeque;

use anyhow::{Ok, Result};
use aoc2022::RegexExtract;
use itertools::Itertools;
use regex::Regex;

const MONKEY_FORMAT: &'static str = r"Monkey \d+:
\s*Starting items: (\d+(?:, \d+)*)
\s*Operation: new = old ([*+]) (old|\d+)
\s*Test: divisible by (\d+)
\s*If true: throw to monkey (\d+)
\s*If false: throw to monkey (\d+)";

#[derive(Clone)]
enum Operation {
    AddConst(u64),
    MulConst(u64),
    Double,
    Square,
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    divisor: u64,
    targets: [usize; 2],
    num_inspections: u64,
}

fn monkey_business(mut monkeys: Vec<Monkey>, rounds: usize, div: u64, rem: u64) -> u64 {
    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys[m].items.pop_front() {
                let afterop = match monkeys[m].op {
                    Operation::AddConst(c) => item + c,
                    Operation::MulConst(c) => item * c,
                    Operation::Double => item + item,
                    Operation::Square => item * item,
                };
                let new = afterop / div % rem;
                let target = monkeys[m].targets[(new % monkeys[m].divisor == 0) as usize];
                monkeys[target].items.push_back(new);
                monkeys[m].num_inspections += 1;
            }
        }
    }

    monkeys.select_nth_unstable_by(1, |a, b| b.num_inspections.cmp(&a.num_inspections));
    monkeys[0].num_inspections * monkeys[1].num_inspections
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day11.txt")?;
    let start = std::time::Instant::now();

    let re = Regex::new(MONKEY_FORMAT)?;
    let monkeys: Vec<Monkey> = re
        .extract_iter(&input)
        .map(|(_, note)| {
            let [start, optype, oparg, div, iftrue, iffalse] = note;
            let op = match optype {
                "+" if oparg == "old" => Operation::Double,
                "+" => Operation::AddConst(oparg.parse()?),
                "*" if oparg == "old" => Operation::Square,
                "*" => Operation::MulConst(oparg.parse()?),
                _ => unreachable!(),
            };

            Ok(Monkey {
                items: start.split(',').map(|s| s.trim().parse()).try_collect()?,
                op,
                divisor: div.parse()?,
                targets: [iffalse.parse()?, iftrue.parse()?],
                num_inspections: 0,
            })
        })
        .try_collect()?;

    let rem = monkeys.iter().map(|m| m.divisor).product();
    let part1 = monkey_business(monkeys.clone(), 20, 3, 1u64 << 63);
    let part2 = monkey_business(monkeys, 10000, 1, rem);

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
