use anyhow::{Ok, Result, Context};
use aoc2022::Priority;
use hashbrown::HashSet;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn astar(
    start: [usize; 3],
    stop: [usize; 2],
    dims: [usize; 3],
    forbidden: &[bool],
) -> Option<usize> {
    let [width, height, period] = dims;
    let h = |[x, y, t]: [usize; 3]| t + stop[0].abs_diff(x) + stop[1].abs_diff(y);
    let valid = |[x, y, t]: [usize; 3]| !forbidden[x + y * width + (t % period) * width * height];
    
    let mut reached = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(Priority(Reverse(h(start)), start));
    while let Some(Priority(_, cur)) = heap.pop() {
        let [x, y, t] = cur;
        if [x, y] == stop {
            return Some(t);
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
            let nx = (x as i64 + dx).clamp(0, width as i64 - 1);
            let ny = (y as i64 + dy).clamp(0, height as i64 - 1);
            let next = [nx as usize, ny as usize, t + 1];
            if valid(next) && reached.insert(next) {
                heap.push(Priority(Reverse(h(next)), next));
            }
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day24.txt")?;
    let start = std::time::Instant::now();

    let lines = input.lines().map(|l| l.as_bytes()).collect_vec();
    let (width, height) = (lines[0].len(), lines.len());

    // Construct forbidden position mask.
    let blizz_period = (width - 2) * (height - 2) / gcd(width - 2, height - 2);
    let mut forbidden = vec![false; width * height * blizz_period];
    let mut set = |x, y, t, v| forbidden[x + y * width + t * width * height] = v;
    for t in 0..blizz_period {
        for x in 0..width {
            set(x, 0, t, true);
            set(x, height - 1, t, true);
        }
        for y in 0..height {
            set(0, y, t, true);
            set(width - 1, y, t, true);
        }
        set(1, 0, t, false);
        set(width - 2, height - 1, t, false);
    }

    for (r, line) in lines.iter().enumerate() {
        for (c, pos) in line.iter().enumerate() {
            let (dx, dy) = match *pos {
                b'<' => (-1, 0),
                b'>' => (1, 0),
                b'^' => (0, -1),
                b'v' => (0, 1),
                _ => continue,
            };

            for t in 0..blizz_period as isize {
                let x = 1 + (c as isize - 1 + t * dx).rem_euclid(width as isize - 2);
                let y = 1 + (r as isize - 1 + t * dy).rem_euclid(height as isize - 2);
                set(x as usize, y as usize, t as usize, true);
            }
        }
    }
    
    let dims = [width, height, blizz_period];
    let there = astar([1, 0, 0], [width - 2, height - 1], dims, &forbidden).context("no path")?;
    let back = astar([width - 2, height - 1, there], [1, 0], dims, &forbidden).context("no path")?;
    let again = astar([1, 0, back], [width - 2, height - 1], dims, &forbidden).context("no path")?;

    println!("part1: {}", there);
    println!("part2: {}", again);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
