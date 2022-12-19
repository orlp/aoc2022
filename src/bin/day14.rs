use anyhow::{Context, Ok, Result};
use itertools::Itertools;

fn parse_coord(coord: &str) -> Result<(usize, usize)> {
    let (x, y) = coord.trim().split_once(",").context("invalid coordinate")?;
    Ok((x.trim().parse()?, y.trim().parse()?))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day14.txt")?;
    let start = std::time::Instant::now();

    let paths_it = input.lines().map(|l| l.split("->").map(parse_coord));
    let paths: Vec<Vec<_>> = paths_it.map(|p| p.try_collect()).try_collect()?;
    let width = 2 * paths.iter().flatten().map(|(x, _y)| x).max().unwrap() + 1;
    let height = paths.iter().flatten().map(|(_x, y)| y).max().unwrap() + 3;
    let mut grid = vec![false; width * height];
    for x in 0..width {
        grid[(height - 1) * width + x] = true;
    }
    for path in &paths {
        for (&(x0, y0), &(x1, y1)) in path.iter().tuple_windows() {
            for x in x0.min(x1)..=x0.max(x1) {
                for y in y0.min(y1)..=y0.max(y1) {
                    grid[y * width + x] = true;
                }
            }
        }
    }

    let (mut max_y, mut part1, mut part2) = (0, 0, 0);
    let mut fall_path = vec![(500usize, 0)];
    'fall_loop: while let Some((sx, sy)) = fall_path.last().copied() {
        for dx in [0, -1, 1] {
            if !grid[(sy + 1) * width + sx.saturating_add_signed(dx)] {
                fall_path.push((sx.saturating_add_signed(dx), sy + 1));
                continue 'fall_loop;
            }
        }

        fall_path.pop();
        max_y = max_y.max(sy);
        grid[sy * width + sx] = true;
        part1 += (max_y < height - 2) as usize;
        part2 += 1;
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
