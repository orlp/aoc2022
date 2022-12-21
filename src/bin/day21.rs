use anyhow::{Ok, Result};
use aoc2022::OptionSomeExt;
use hashbrown::HashMap;
use itertools::Itertools;

use z3::ast::{Ast, Real, Int};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day21.txt")?;
    let start = std::time::Instant::now();

    // Yes, z3. I refuse to assume the input only contains humn once, and I can
    // not be bothered making a single-variable rational diophantine equation
    // solver.
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut z3_monkeys: HashMap<&str, Real> = HashMap::new();
    let mut z3_monkey = |name| {
        let entry = z3_monkeys.entry(name);
        entry.or_insert_with_key(|k| Real::new_const(&ctx, *k)).clone()
    };

    let mut values: HashMap<&str, i64> = HashMap::new();
    let mut provides_to: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut formulae: HashMap<&str, (&str, char, &str, usize)> = HashMap::new();
    let mut ready: Vec<&str> = Vec::new();

    for line in input.lines() {
        let (name, rest) = line.split_once(':').some()?;
        let z3_name = z3_monkey(name);

        if rest.trim().chars().next().some()?.is_numeric() {
            let value = rest.trim().parse()?;
            values.insert(name, value);
            ready.push(name);
            if name != "humn" {
                solver.assert(&z3_name._eq(&Real::from_int(&Int::from_i64(&ctx, value))));
            }
            continue;
        }

        let (left, op, right) = rest.split_ascii_whitespace().collect_tuple().some()?;
        provides_to.entry(left).or_default().push(name);
        provides_to.entry(right).or_default().push(name);
        formulae.insert(name, (left, op.chars().next().unwrap(), right, 2));

        let (z3_left, z3_right) = (z3_monkey(left), z3_monkey(right));
        if name == "root" {
            solver.assert(&z3_left._eq(&z3_right));
        } else {
            match op {
                "+" => solver.assert(&z3_name._eq(&(z3_left + z3_right))),
                "-" => solver.assert(&z3_name._eq(&(z3_left - z3_right))),
                "*" => solver.assert(&z3_name._eq(&(z3_left * z3_right))),
                "/" => solver.assert(&z3_name._eq(&(z3_left / z3_right))),
                _ => anyhow::bail!("unknown formula"),
            }
        }
    }

    while let Some(monkey) = ready.pop() {
        let value = match formulae.remove(monkey) {
            Some((left, '+', right, _)) => values[left] + values[right],
            Some((left, '-', right, _)) => values[left] - values[right],
            Some((left, '*', right, _)) => values[left] * values[right],
            Some((left, '/', right, _)) => values[left] / values[right],
            Some(_) => unreachable!(),
            None => values[monkey],
        };
        values.insert(monkey, value);

        for target in provides_to.remove(monkey).unwrap_or_default() {
            formulae.get_mut(target).unwrap().3 -= 1;
            if formulae[target].3 == 0 {
                ready.push(target);
            }
        }
    }
    
    solver.check();
    let model = solver.get_model().some()?;
    let ret = model.eval(&z3_monkey("humn"), false).some()?;

    println!("part1: {}", values["root"]);
    println!("part2: {}", ret);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
