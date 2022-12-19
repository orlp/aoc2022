use std::collections::BinaryHeap;

use anyhow::{Ok, Result};
use aoc2022::{Priority, RegexExtract};
use hashbrown::HashSet;
use itertools::Itertools;
use regex::Regex;

const BLUEPRINT_REGEX: &'static str = "\
Blueprint (\\d+): \
Each ore robot costs (\\d+) ore. \
Each clay robot costs (\\d+) ore. \
Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
Each geode robot costs (\\d+) ore and (\\d+) obsidian.";

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug)]
struct Blueprint {
    ore_costs: [u32; 4],
    max_ore_cost: u32,
    obsidian_clay_cost: u32,
    geode_obsidian_cost: u32,
}

impl Blueprint {
    pub fn best_num_geodes(&self, minutes: u32) -> u32 {
        let mut best = 0;
        let mut executions = BinaryHeap::new();
        executions.push(Priority(0, Execution::new()));
        let mut seen = HashSet::new();
        while let Some(ex) = executions.pop() {
            let choices = (0..4).map(|r| ex.1.build_robot(r, self, minutes)).flatten();
            for next in choices {
                let upper_bound = next.geode_upper_bound(self, minutes);
                if upper_bound > best {
                    best = best.max(next.geode_lower_bound(self, minutes));
                    if next.minutes < minutes && !seen.contains(&next) {
                        seen.insert(next.clone());
                        executions.push(Priority(upper_bound, next));
                    }
                }
            }
        }
        best
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Execution {
    robots: [u32; 4],
    resources: [u32; 4],
    minutes: u32,
}

impl Execution {
    fn new() -> Self {
        Self {
            robots: [1, 0, 0, 0],
            resources: [0; 4],
            minutes: 0,
        }
    }

    // Computes how many resources we mine given initial robots and assuming we
    // build an extra robot each minute for the first extra_robots minutes.
    fn max_mining(&self, resource: usize, extra_robots: u32, time_left: u32) -> u32 {
        let b = self.robots[resource];
        let x = extra_robots.min(time_left);
        // b + (b + 1) + (b + 2) + ... + (b + x - 1) + (b + x) + ... + (b + x)
        //       x*(2b + x - 1) / 2                  |    (b + x) * (m - x)
        self.resources[resource] + x * (2 * b + x - 1) / 2 + (b + x) * (time_left - x)
    }

    pub fn geode_lower_bound(&self, bp: &Blueprint, max_minutes: u32) -> u32 {
        assert!(self.minutes <= max_minutes);
        let ore_afford = self.resources[ORE] / bp.ore_costs[GEODE];
        let obs_afford = self.resources[OBSIDIAN] / bp.geode_obsidian_cost;
        self.max_mining(GEODE, ore_afford.min(obs_afford), max_minutes - self.minutes)
    }

    pub fn geode_upper_bound(&self, bp: &Blueprint, max_minutes: u32) -> u32 {
        // Let's first upper bound how much ore we could ultimately have, by
        // assuming we could go into debt, and always building ore machines if
        // they'd pay off for themselves, which is after cost + 1 minutes.
        // Then, again allowing debt, spend all this ore on clay, which in turn
        // gets spent on obsidian and geodes, ignoring ore costs.
        let m = max_minutes - self.minutes;
        let extra_ore_bots = m.saturating_sub(bp.ore_costs[ORE] + 1);
        let max_ore = self.max_mining(ORE, extra_ore_bots, m) - extra_ore_bots * bp.ore_costs[ORE];
        let max_clay = self.max_mining(CLAY, max_ore / bp.ore_costs[CLAY], m);
        let max_obsidian = self.max_mining(OBSIDIAN, max_clay / bp.obsidian_clay_cost, m);
        self.max_mining(GEODE, max_obsidian / bp.geode_obsidian_cost, m)
    }

    pub fn build_robot(&self, resource: usize, bp: &Blueprint, max_minutes: u32) -> Option<Execution> {
        // Don't build if we're already gathering enough to sustain the factory.
        if resource == ORE && self.robots[ORE] >= bp.max_ore_cost
            || resource == CLAY && self.robots[CLAY] >= bp.obsidian_clay_cost
            || resource == OBSIDIAN && self.robots[OBSIDIAN] >= bp.geode_obsidian_cost
        {
            return None;
        }

        let costs = [
            bp.ore_costs[resource],
            bp.obsidian_clay_cost * (resource == OBSIDIAN) as u32,
            bp.geode_obsidian_cost * (resource == GEODE) as u32,
        ];
        let [ore_t, clay_t, obs_t] = [ORE, CLAY, OBSIDIAN].map(|r| {
            let resources_needed = costs[r].saturating_sub(self.resources[r]);
            if resources_needed > 0 {
                (resources_needed + self.robots[r] - 1).checked_div(self.robots[r])
            } else {
                Some(0)
            }
        });
        let mins = 1 + ore_t?.max(clay_t?).max(obs_t?);
        let mut ret = self.clone();
        for r in 0..4 {
            ret.resources[r] += mins * ret.robots[r] - costs.get(r).unwrap_or(&0)
        }
        ret.minutes += mins;
        ret.robots[resource] += 1;
        (ret.minutes <= max_minutes).then_some(ret)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day19.txt")?;
    let start = std::time::Instant::now();

    let re = Regex::new(BLUEPRINT_REGEX)?;
    let blueprints: Vec<_> = re
        .extract_iter(&input)
        .map(|(_, bp)| {
            let [_id, ore, clay, obs_ore, obs, geo_ore, geo] = bp.map(str::parse);
            let ore_costs = [ore?, clay?, obs_ore?, geo_ore?];
            Ok(Blueprint {
                ore_costs,
                max_ore_cost: ore_costs[0].max(ore_costs[1]).max(ore_costs[2]).max(ore_costs[3]),
                obsidian_clay_cost: obs?,
                geode_obsidian_cost: geo?,
            })
        })
        .try_collect()?;

    let part1: u32 = blueprints
        .iter()
        .enumerate()
        .map(|(i, bp)| bp.best_num_geodes(24) as u32 * (i as u32 + 1))
        .sum();
    let part2: u32 = blueprints[..3].iter().map(|bp| bp.best_num_geodes(32) as u32).product();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    println!("time: {:?}", start.elapsed());
    Ok(())
}
