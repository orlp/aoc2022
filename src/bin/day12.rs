use anyhow::{bail, Context, Ok, Result};
use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::VecDeque;

fn bfs(grid: &[u8], width: usize, start: u8, target: u8, forwards: bool) -> Result<usize> {
    let height = grid.len() / width;
    let startpos = grid.iter().position(|c| *c == start).context("no start")?;
    let elevation = |pos| match grid[pos] {
        b'S' => b'a',
        b'E' => b'z',
        c => c,
    };

    let mut to_visit: VecDeque<_> = [(startpos, 0)].into_iter().collect();
    let mut seen = HashSet::new();
    while let Some((pos, steps)) = to_visit.pop_front() {
        if grid[pos] == target {
            return Ok(steps);
        }

        let (x, y) = (pos % width, pos / width);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let (Some(nx), Some(ny)) = (x.checked_add_signed(dx), y.checked_add_signed(dy)) {
                let npos = ny * width + nx;
                if nx < width && ny < height && !seen.contains(&npos) {
                    let back = elevation(pos) <= elevation(npos) + 1;
                    let forth = elevation(npos) <= elevation(pos) + 1;
                    if forwards && forth || !forwards && back {
                        to_visit.push_back((npos, steps + 1));
                        seen.insert(npos);
                    }
                }
            }
        }
    }

    bail!("target not found")
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day12.txt")?;
    let start = std::time::Instant::now();

    let width = input.lines().next().context("no line")?.len();
    let grid = input.lines().join("").into_bytes();
    println!("part1: {}", bfs(&grid, width, b'S', b'E', true)?);
    println!("part2: {}", bfs(&grid, width, b'E', b'a', false)?);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
