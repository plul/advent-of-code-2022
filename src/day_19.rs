//! Day 19: Not Enough Minerals
//!
//! https://adventofcode.com/2022/day/19

use std::cmp::max;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::ops::SubAssign;

pub fn part_1(input: &str) -> i64 {
    let blueprints = parser::parse(input);
    let minutes = 24;

    blueprints
        .iter()
        .map(|&blueprint| {
            let mut cache = HashMap::default();
            let geodes = max_geodes_dfs(
                blueprint,
                Robots::default(),
                Resources::default(),
                minutes,
                &mut cache,
            );
            (blueprint, geodes)
        })
        .map(|(blueprint, max_geodes)| quality_level(blueprint.id, max_geodes))
        .sum()
}

pub fn part_2(input: &str) -> i64 {
    let blueprints = parser::parse(input);
    let minutes = 32;

    blueprints
        .iter()
        .inspect(|b| {
            dbg!(b);
        })
        .take(3)
        .map(|&blueprint| {
            let mut cache = HashMap::default();
            max_geodes_dfs(
                blueprint,
                Robots::default(),
                Resources::default(),
                minutes,
                &mut cache,
            )
        })
        .product()
}

type Minutes = i64;

/// DFS
fn max_geodes_dfs(
    blueprint: Blueprint,
    robots: Robots,
    resources: Resources,
    minutes: Minutes,
    cache: &mut HashMap<(Robots, Resources, Minutes), i64>,
) -> i64 {
    debug_assert!(minutes >= 0);
    debug_assert!(robots.ore > 0);
    debug_assert!(resources.ore >= 0);
    debug_assert!(resources.clay >= 0);
    debug_assert!(resources.obsidian >= 0);
    debug_assert!(resources.geode >= 0);

    if minutes == 0 {
        return resources.geode;
    }

    let cache_cutoff = 5;

    // Best if not building any more robots.
    let mut best = resources.geode + robots.geode * minutes;

    'ore_robot: {
        if blueprint.ore_robot_ore_cost > 0 && robots.ore == 0 {
            break 'ore_robot;
        }
        let mut robots = robots;
        let mut resources = resources;
        let mut minutes = minutes;
        let minutes_gathering = max(0, blueprint.ore_robot_ore_cost - resources.ore).div_ceil(robots.ore);
        minutes -= minutes_gathering + 1;
        if minutes < 0 {
            break 'ore_robot;
        }
        resources += Resources {
            ore: robots.ore * (minutes_gathering + 1),
            clay: robots.clay * (minutes_gathering + 1),
            obsidian: robots.obsidian * (minutes_gathering + 1),
            geode: robots.geode * (minutes_gathering + 1),
        };
        resources -= Resources {
            ore: blueprint.ore_robot_ore_cost,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        robots += Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };

        let cached = if minutes >= cache_cutoff {
            cache.get(&(robots, resources, minutes)).cloned()
        } else {
            None
        };
        let result = if let Some(result) = cached {
            result
        } else {
            max_geodes_dfs(blueprint, robots, resources, minutes, cache)
        };
        if minutes >= cache_cutoff && cached.is_none() {
            cache.insert((robots, resources, minutes), result);
        };
        best = max(best, result);
    }

    'clay_robot: {
        if blueprint.clay_robot_ore_cost > 0 && robots.ore == 0 {
            break 'clay_robot;
        }
        let mut robots = robots;
        let mut resources = resources;
        let mut minutes = minutes;
        let minutes_gathering = max(0, blueprint.clay_robot_ore_cost - resources.ore).div_ceil(robots.ore);
        minutes -= minutes_gathering + 1;
        if minutes < 0 {
            break 'clay_robot;
        }
        resources += Resources {
            ore: robots.ore * (minutes_gathering + 1),
            clay: robots.clay * (minutes_gathering + 1),
            obsidian: robots.obsidian * (minutes_gathering + 1),
            geode: robots.geode * (minutes_gathering + 1),
        };
        resources -= Resources {
            ore: blueprint.clay_robot_ore_cost,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        robots += Robots {
            ore: 0,
            clay: 1,
            obsidian: 0,
            geode: 0,
        };

        let cached = if minutes >= cache_cutoff {
            cache.get(&(robots, resources, minutes)).cloned()
        } else {
            None
        };
        let result = if let Some(result) = cached {
            result
        } else {
            max_geodes_dfs(blueprint, robots, resources, minutes, cache)
        };
        if minutes >= cache_cutoff && cached.is_none() {
            cache.insert((robots, resources, minutes), result);
        };
        best = max(best, result);
    }

    'obsidian_robot: {
        if (blueprint.obsidian_robot_ore_cost > 0 && robots.ore == 0)
            || (blueprint.obsidian_robot_clay_cost > 0 && robots.clay == 0)
        {
            break 'obsidian_robot;
        }

        let mut robots = robots;
        let mut resources = resources;
        let mut minutes = minutes;
        let minutes_gathering_ore =
            max(0, blueprint.obsidian_robot_ore_cost - resources.ore).div_ceil(robots.ore);
        let minutes_gathering_clay =
            max(0, blueprint.obsidian_robot_clay_cost - resources.clay).div_ceil(robots.clay);
        let minutes_gathering = max(minutes_gathering_ore, minutes_gathering_clay);
        minutes -= minutes_gathering + 1;
        if minutes < 0 {
            break 'obsidian_robot;
        }
        resources += Resources {
            ore: robots.ore * (minutes_gathering + 1),
            clay: robots.clay * (minutes_gathering + 1),
            obsidian: robots.obsidian * (minutes_gathering + 1),
            geode: robots.geode * (minutes_gathering + 1),
        };
        resources -= Resources {
            ore: blueprint.obsidian_robot_ore_cost,
            clay: blueprint.obsidian_robot_clay_cost,
            obsidian: 0,
            geode: 0,
        };
        robots += Robots {
            ore: 0,
            clay: 0,
            obsidian: 1,
            geode: 0,
        };

        let cached = if minutes >= cache_cutoff {
            cache.get(&(robots, resources, minutes)).cloned()
        } else {
            None
        };
        let result = if let Some(result) = cached {
            result
        } else {
            max_geodes_dfs(blueprint, robots, resources, minutes, cache)
        };
        if minutes >= cache_cutoff && cached.is_none() {
            cache.insert((robots, resources, minutes), result);
        };
        best = max(best, result);
    }

    'geode_robot: {
        if (blueprint.geode_robot_ore_cost > 0 && robots.ore == 0)
            || (blueprint.geode_robot_obsidian_cost > 0 && robots.obsidian == 0)
        {
            break 'geode_robot;
        }

        let mut robots = robots;
        let mut resources = resources;
        let mut minutes = minutes;
        let minutes_gathering_ore =
            max(0, blueprint.geode_robot_ore_cost - resources.ore).div_ceil(robots.ore);
        let minutes_gathering_obsidian =
            max(0, blueprint.geode_robot_obsidian_cost - resources.obsidian).div_ceil(robots.obsidian);
        let minutes_gathering = max(minutes_gathering_ore, minutes_gathering_obsidian);
        minutes -= minutes_gathering + 1;
        if minutes < 0 {
            break 'geode_robot;
        }
        resources += Resources {
            ore: robots.ore * (minutes_gathering + 1),
            clay: robots.clay * (minutes_gathering + 1),
            obsidian: robots.obsidian * (minutes_gathering + 1),
            geode: robots.geode * (minutes_gathering + 1),
        };
        resources -= Resources {
            ore: blueprint.geode_robot_ore_cost,
            clay: 0,
            obsidian: blueprint.geode_robot_obsidian_cost,
            geode: 0,
        };
        robots += Robots {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 1,
        };

        let cached = if minutes >= cache_cutoff {
            cache.get(&(robots, resources, minutes)).cloned()
        } else {
            None
        };
        let result = if let Some(result) = cached {
            result
        } else {
            max_geodes_dfs(blueprint, robots, resources, minutes, cache)
        };
        if minutes >= cache_cutoff && cached.is_none() {
            cache.insert((robots, resources, minutes), result);
        };
        best = max(best, result);
    }

    best
}

fn quality_level(blueprint_id: i64, max_geodes: i64) -> i64 {
    blueprint_id * max_geodes
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Robots {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}
impl Default for Robots {
    fn default() -> Self {
        Self {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}
impl AddAssign for Robots {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
struct Resources {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geode: i64,
}
impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}
impl SubAssign for Resources {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: i64,
    ore_robot_ore_cost: i64,
    clay_robot_ore_cost: i64,
    obsidian_robot_ore_cost: i64,
    obsidian_robot_clay_cost: i64,
    geode_robot_ore_cost: i64,
    geode_robot_obsidian_cost: i64,
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Blueprint> {
        all_consuming(terminated(
            separated_list1(line_ending, parse_blueprint),
            multispace0,
        ))(s)
        .unwrap()
        .1
    }

    fn parse_blueprint(s: &str) -> IResult<&str, Blueprint> {
        let (s, id) = delimited(tag("Blueprint "), u64, char(':'))(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, ore_robot_ore_cost) = delimited(tag("Each ore robot costs "), u64, tag(" ore."))(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, clay_robot_ore_cost) = delimited(tag("Each clay robot costs "), u64, tag(" ore."))(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, (obsidian_robot_ore_cost, obsidian_robot_clay_cost)) = delimited(
            tag("Each obsidian robot costs "),
            separated_pair(u64, tag(" ore and "), u64),
            tag(" clay."),
        )(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, (geode_robot_ore_cost, geode_robot_obsidian_cost)) = delimited(
            tag("Each geode robot costs "),
            separated_pair(u64, tag(" ore and "), u64),
            tag(" obsidian."),
        )(s)?;

        let blueprint = Blueprint {
            id: id as i64,
            ore_robot_ore_cost: ore_robot_ore_cost as i64,
            clay_robot_ore_cost: clay_robot_ore_cost as i64,
            obsidian_robot_ore_cost: obsidian_robot_ore_cost as i64,
            obsidian_robot_clay_cost: obsidian_robot_clay_cost as i64,
            geode_robot_ore_cost: geode_robot_ore_cost as i64,
            geode_robot_obsidian_cost: geode_robot_obsidian_cost as i64,
        };
        Ok((s, blueprint))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 33);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 56 * 62);
}
