use anyhow::{Context, Result};
use itertools::{izip, Itertools};

/// Computes how many trees we can see in direction x1, ..., xn for all y=0..h
/// in grid using given strides xs, xy, as well as whether this tree can be seen
/// from the edge of the grid in this direction.
///
/// For each row we maintain a stack of tree indices in strictly descending
/// order of height while scanning from the *end* of the row. Each time we
/// encounter a new tree we can remove all trees higher or equal on the stack,
/// as they're blocked by this tree as viewed from the start (and thus we know
/// their viewing distance). After doing this the new tree is the smallest tree
/// and can thus be pushed on the stack. Any trees remaining on the stack at the
/// end are viewable from the edge at the row start.
fn view(grid: &[u8], x1: usize, xn: usize, h: usize, xs: usize, ys: usize) -> Vec<(u64, bool)> {
    let dx = if x1 < xn { 1 } else { -1 };
    let mut result = vec![(0, false); grid.len()];
    let mut store_result = |xi, y, r| result[(x1 + (dx * xi) as usize) * xs + y * ys] = r;
    let height = |xi, y| grid[(x1 + (dx * xi) as usize) * xs + y * ys];

    let mut stack = Vec::new();
    for y in 0..h {
        for xi in (0..(1 + x1.abs_diff(xn) as i64)).rev() {
            while stack.last().map(|xj| height(xi, y) >= height(*xj, y)).unwrap_or(false) {
                let xj = stack.pop().unwrap();
                store_result(xj, y, (xj.abs_diff(xi), false));
            }
            stack.push(xi);
        }

        for xi in stack.drain(..) {
            store_result(xi, y, (xi.unsigned_abs(), true));
        }
    }

    result
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day08.txt")?;
    let start = std::time::Instant::now();

    let grid = input.lines().flat_map(|l| l.trim().bytes()).collect_vec();
    let w = input.lines().next().context("no input")?.trim().len();
    let h = grid.len() / w;
    let left = view(&grid, 0, w - 1, h, 1, w);
    let right = view(&grid, w - 1, 0, h, 1, w);
    let down = view(&grid, 0, h - 1, w, w, 1);
    let up = view(&grid, h - 1, 0, w, w, 1);
    let part1 = izip!(&left, &right, &down, &up).filter(|(l, r, d, u)| l.1 | r.1 | d.1 | u.1);
    let part2 = izip!(&left, &right, &down, &up).map(|(l, r, u, d)| l.0 * r.0 * d.0 * u.0);

    println!("part1: {}", part1.count());
    println!("part2: {}", part2.max().unwrap());
    println!("time: {:?}", start.elapsed());
    Ok(())
}
