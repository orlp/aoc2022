use std::collections::BinaryHeap;
use std::str::FromStr;

use anyhow::{Ok, Result};
use aoc2022::{OptionSomeExt, Priority};
use hashbrown::HashSet;
use itertools::Itertools;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBS: usize = 2;
const GEODE: usize = 3;

#[derive(Debug)]
struct Blueprint {
    ore_costs: [u32; 4],
    max_ore_cost: u32,
    obsidian_clay_cost: u32,
    geode_obsidian_cost: u32,
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split_ascii_whitespace().filter_map(|s| s.parse().ok());
        let (ore, clay, obs_ore, obs, geo_ore, geo) = nums.collect_tuple().some()?;
        Ok(Blueprint {
            ore_costs: [ore, clay, obs_ore, geo_ore],
            max_ore_cost: ore.max(clay).max(obs_ore).max(geo_ore),
            obsidian_clay_cost: obs,
            geode_obsidian_cost: geo,
        })
    }
}

impl Blueprint {
    pub fn best_num_geodes(&self, minutes: u32) -> u32 {
        let mut best = 0;
        let mut executions = BinaryHeap::new();
        executions.push(Priority(1, Execution::new()));
        let mut seen = HashSet::new();
        while let Some(Priority(upper, ex)) = executions.pop() {
            if upper <= best {
                break;
            }
            let choices = (0..4).map(|r| ex.build_robot(r, self, minutes)).flatten();
            for next in choices {
                let next_upper = next.geode_upper_bound(self, minutes);
                if next_upper > best {
                    best = best.max(next.geode_lower_bound(self, minutes));
                    if next.minutes < minutes && !seen.contains(&next) {
                        seen.insert(next.clone());
                        executions.push(Priority(next_upper, next));
                    }
                }
            }
        }

        best
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Default)]
struct Execution {
    robots: [u32; 4],
    resources: [u32; 4],
    minutes: u32,
}

impl Execution {
    fn new() -> Self {
        let mut slf = Self::default();
        slf.robots[0] = 1;
        slf
    }

    pub fn geode_lower_bound(&self, bp: &Blueprint, max_mins: u32) -> u32 {
        // We only construct geode bots.
        let mut robots = self.robots;
        let mut res = self.resources;
        for _m in self.minutes..max_mins {
            let new_bot = res[ORE] >= bp.ore_costs[GEODE] && res[OBS] >= bp.geode_obsidian_cost;
            for r in 0..4 {
                res[r] += robots[r];
            }
            res[ORE] -= bp.ore_costs[GEODE] * new_bot as u32;
            res[OBS] -= new_bot as u32 * bp.geode_obsidian_cost;
            robots[GEODE] += new_bot as u32;
        }

        res[GEODE]
    }

    pub fn geode_upper_bound(&self, bp: &Blueprint, max_mins: u32) -> u32 {
        // We greedily build robots, but the costs for one type of robot are
        // not subtracted from the pool of resources of the other robots, and we
        // can build multiple robot types at once.
        let mut robots = self.robots;
        let mut ores_for = [self.resources[0]; 4];
        let [_, mut clay, mut obs, mut geodes] = self.resources;
        for _m in self.minutes..max_mins {
            let new_bot = [
                ores_for[ORE] >= bp.ore_costs[ORE],
                ores_for[CLAY] >= bp.ore_costs[CLAY],
                ores_for[OBS] >= bp.ore_costs[OBS] && clay >= bp.obsidian_clay_cost,
                ores_for[GEODE] >= bp.ore_costs[GEODE] && obs >= bp.geode_obsidian_cost,
            ];

            for r in 0..4 {
                ores_for[r] += robots[ORE] - new_bot[r] as u32 * bp.ore_costs[r];
            }
            clay += robots[CLAY] - new_bot[OBS] as u32 * bp.obsidian_clay_cost;
            obs += robots[OBS] - new_bot[GEODE] as u32 * bp.geode_obsidian_cost;
            geodes += robots[GEODE];

            for r in 0..4 {
                robots[r] += new_bot[r] as u32;
            }
        }

        geodes
    }

    pub fn build_robot(&self, resource: usize, bp: &Blueprint, max_mins: u32) -> Option<Execution> {
        let have_enough_already = match resource {
            ORE => self.robots[ORE] >= bp.max_ore_cost,
            CLAY => self.robots[CLAY] >= bp.obsidian_clay_cost,
            OBS => self.robots[OBS] >= bp.geode_obsidian_cost,
            _ => false,
        };
        let costs = [
            bp.ore_costs[resource],
            bp.obsidian_clay_cost * (resource == OBS) as u32,
            bp.geode_obsidian_cost * (resource == GEODE) as u32,
        ];
        let [ore_t, clay_t, obs_t] = [ORE, CLAY, OBS].map(|r| {
            if costs[r] <= self.resources[r] {
                return Some(0);
            }
            (costs[r] - self.resources[r] + self.robots[r] - 1).checked_div(self.robots[r])
        });
        let delay = 1 + ore_t?.max(clay_t?).max(obs_t?);

        let mut ret = self.clone();
        for r in 0..4 {
            ret.resources[r] += delay * ret.robots[r] - costs.get(r).unwrap_or(&0)
        }
        ret.minutes += delay;
        ret.robots[resource] += 1;
        (!have_enough_already && ret.minutes <= max_mins).then_some(ret)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day19.txt")?;
    let start = std::time::Instant::now();

    let bps: Vec<Blueprint> = input.lines().map(|l| l.parse()).try_collect()?;
    let p1_best = bps.iter().map(|bp| bp.best_num_geodes(24));
    let part1: u32 = p1_best.enumerate().map(|(i, b)| b * (i as u32 + 1)).sum();
    let part2: u32 = bps[..3].iter().map(|bp| bp.best_num_geodes(32)).product();

    println!("part1: {part1}");
    println!("part2: {part2}");
    println!("time: {:?}", start.elapsed());
    Ok(())
}
