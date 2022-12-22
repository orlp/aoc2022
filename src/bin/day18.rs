use anyhow::{Ok, Result};
use aoc2022::OptionSomeExt;
use hashbrown::{HashMap, HashSet};
use itertools::{iproduct, Itertools};

fn offset(mut xyz: [i64; 3], dim: usize, offset: i64) -> [i64; 3] {
    xyz[dim] += offset;
    xyz
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day18.txt")?;
    let start = std::time::Instant::now();

    let mut faces: HashMap<[i64; 3], usize> = HashMap::new();
    let mut cubes: HashSet<[i64; 3]> = HashSet::new();
    for line in input.lines() {
        let xyz = line.split(',').map(|n| n.parse::<i64>());
        let (x, y, z) = xyz.collect_tuple().some()?;
        let xyz = [x? * 2, y? * 2, z? * 2];
        cubes.insert(xyz);
        for dim in 0..3 {
            *faces.entry(offset(xyz, dim, -1)).or_default() += 1;
            *faces.entry(offset(xyz, dim, 1)).or_default() += 1;
        }
    }

    let part1 = faces.values().filter(|f| **f == 1).count();

    // Walk the air around the surface.
    let mut part2 = 0;
    let first_cube = *cubes.iter().min_by_key(|[x, _y, _z]| x).some()?;
    let mut to_visit = vec![(offset(first_cube, 0, -2), true)];
    let mut seen = HashSet::new();
    let mut air_neighbors = Vec::new();
    while let Some((air, touches_surface)) = to_visit.pop() {
        if seen.insert(air) {
            let mut cubes_facing_air = 0;
            air_neighbors.clear();
            for neighbor in iproduct!((0..3), [-2, 2]).map(|(d, s)| offset(air, d, s)) {
                if cubes.contains(&neighbor) {
                    cubes_facing_air += 1;
                } else {
                    air_neighbors.push(neighbor);
                }
            }

            part2 += cubes_facing_air;
            if touches_surface || cubes_facing_air > 0 {
                to_visit.extend(air_neighbors.iter().map(|n| (*n, cubes_facing_air > 0)));
            }
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
