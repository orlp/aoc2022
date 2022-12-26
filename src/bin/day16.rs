use std::cmp::Reverse;
use std::collections::BinaryHeap;

use anyhow::{Ok, Result};
use aoc2022::{OptionSomeExt, Priority, RegexExtract};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

struct Valve<'s> {
    flow: u32,
    neighbors: Vec<&'s str>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pressure: u32,
    opened: u64,
    pos: [usize; 2],
    time: [u32; 2],
}

impl State {
    fn upper_bound(&self, greedy_best_valves: &[Vec<(usize, u32, u32)>]) -> u32 {
        let [mut max_t, mut min_t] = self.time;
        let mut bound = self.pressure;
        'next_valve: loop {
            for (i, mindist, f) in &greedy_best_valves[max_t as usize] {
                if self.opened & (1 << i) == 0 {
                    max_t -= mindist;
                    bound += f * max_t as u32;
                    if max_t < min_t {
                        (min_t, max_t) = (max_t, min_t);
                    }
                    continue 'next_valve;
                }
            }

            return bound;
        }
    }
}

fn max_pressure_release(
    start: usize,
    edges: &[Vec<(usize, u32)>],
    flows: &[u32],
    greedy_best_valves: &[Vec<(usize, u32, u32)>],
    time: [u32; 2],
) -> u32 {
    let init = State {
        pressure: 0,
        opened: 1 << start,
        pos: [start, start],
        time,
    };

    let mut seen = HashSet::new();
    let mut best = 0;
    let mut paths = BinaryHeap::new();
    paths.push(Priority(u32::MAX, init));
    while let Some(Priority(upper, cur)) = paths.pop() {
        if upper <= best {
            return best;
        }

        if !seen.insert(State { pressure: 0, ..cur }) {
            continue;
        }

        for (next, edge_len) in &edges[cur.pos[0]] {
            if cur.time[0] > *edge_len && cur.opened & (1 << next) == 0 {
                let new_time = cur.time[0] - edge_len;
                let mut next_state = State {
                    pressure: cur.pressure + flows[*next] * new_time,
                    opened: cur.opened | (1 << *next),
                    pos: [*next, cur.pos[1]],
                    time: [new_time, cur.time[1]],
                };
                if next_state.time[0] < next_state.time[1] {
                    next_state.pos.swap(0, 1);
                    next_state.time.swap(0, 1);
                }
                best = best.max(next_state.pressure);
                let upper = next_state.upper_bound(greedy_best_valves);
                if upper > best {
                    paths.push(Priority(upper, next_state));
                }
            }
        }
    }
    best
}

fn floyd_warshall(dists: &mut [u32], n: usize) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dists[i + j * n] =
                    dists[i + j * n].min(dists[i + k * n].saturating_add(dists[k + j * n]));
            }
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day16.txt")?;
    let start = std::time::Instant::now();

    let mut valves = Vec::new();
    let mut ids = HashMap::new();
    let re = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z]{2}(?:, [A-Z]{2})*)",
    )?;
    for line in input.lines() {
        let [name, flow, neighbors] = re.extract(line).some()?.1;
        ids.insert(name, valves.len());
        valves.push(Valve {
            flow: flow.parse()?,
            neighbors: neighbors.split(", ").collect_vec(),
        });
    }
    let n = valves.len();
    assert!(n <= 64);

    let mut dists = vec![u32::MAX; n * n];
    for (i, v) in valves.iter().enumerate() {
        dists[i + i * n] = 0;
        for neighbor in &v.neighbors {
            dists[i + ids[neighbor] * n] = 1;
        }
    }
    floyd_warshall(&mut dists, n);

    let direct_connections = (0..n).map(|from| {
        let nonzero_flow = (0..n).flat_map(|to| {
            let dist = dists[from + to * n];
            let valid = (from == ids["AA"] || valves[from].flow > 0 && valves[to].flow > 0)
                && dist < u32::MAX;
            valid.then_some((to, dist + 1))
        });
        nonzero_flow
            .sorted_by_key(|(to, _)| Reverse(valves[*to].flow))
            .collect_vec()
    });
    let edges = direct_connections.collect_vec();
    let flows = valves.iter().map(|v| v.flow).collect_vec();
    let greedy_best_valves = (0..=30)
        .map(|t| {
            // Order valves by payoff given our remaining time t.
            let iflows = flows.iter().copied().enumerate();
            let mut order = iflows
                .map(|(i, f)| {
                    let mindist = (0..n)
                        .filter(|j| i != *j && valves[*j].flow > 0)
                        .map(|j| dists[i + j * n] + 1)
                        .min()
                        .unwrap();
                    (i, mindist, f)
                })
                .collect_vec();
            order.sort_by_key(|(_i, d, f)| Reverse(f * (t - d)));
            order.retain(|(_i, d, f)| t > *d && *f > 0);
            order
        })
        .collect_vec();
    let part1 = max_pressure_release(ids["AA"], &edges, &flows, &greedy_best_valves, [30, 0]);
    let part2 = max_pressure_release(ids["AA"], &edges, &flows, &greedy_best_valves, [26, 26]);

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
