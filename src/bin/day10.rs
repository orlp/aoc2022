use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day10.txt")?;
    let start = std::time::Instant::now();

    let mut value = 1;
    let mut cycle = 0;
    let mut part1 = 0;
    let mut part2 = String::with_capacity(41 * 6);
    for line in input.lines() {
        let (cycles, inc) = match line.trim().split_once(' ').unwrap_or((line, "")) {
            ("noop", "") => (1, 0),
            ("addx", n) => (2, n.parse()?),
            _ => anyhow::bail!("unknown instruction"),
        };

        for _ in 0..cycles {
            let pixel_on = (cycle as i64 % 40).abs_diff(value) <= 1;
            cycle += 1;
            part1 += (cycle % 40 == 20) as i64 * cycle as i64 * value;
            part2.push(if pixel_on { '#' } else { '.' });
            if cycle % 40 == 0 {
                part2.push('\n');
            }
        }
        value += inc;
    }

    println!("part1: {}", part1);
    print!("part2:\n{}", part2);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
