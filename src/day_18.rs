//! Day 18: Boiling Boulders
//!
//! https://adventofcode.com/2022/day/18

use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let lava_cubes = parser::parse(input);
    let droplet = Droplet {
        lava_cubes: lava_cubes.into_iter().collect(),
    };

    droplet
        .lava_cubes
        .iter()
        .flat_map(|c| c.neighbors().filter(|n| !droplet.lava_cubes.contains(n)))
        .count()
}

pub fn part_2(input: &str) -> usize {
    let lava_cubes = parser::parse(input);
    let droplet = Droplet {
        lava_cubes: lava_cubes.into_iter().collect(),
    };
    let min_x = droplet.lava_cubes.iter().map(|c| c.0).min().unwrap() - 1;
    let min_y = droplet.lava_cubes.iter().map(|c| c.1).min().unwrap() - 1;
    let min_z = droplet.lava_cubes.iter().map(|c| c.2).min().unwrap() - 1;
    let max_x = droplet.lava_cubes.iter().map(|c| c.0).max().unwrap() + 1;
    let max_y = droplet.lava_cubes.iter().map(|c| c.1).max().unwrap() + 1;
    let max_z = droplet.lava_cubes.iter().map(|c| c.2).max().unwrap() + 1;

    // DFS to find all cubes of air surrounding droplet.
    let mut outside_air: HashSet<Cube> = HashSet::new();
    let mut stack: Vec<Cube> = Vec::new();
    let start = Cube(min_x, min_y, min_z);
    debug_assert!(!droplet.lava_cubes.contains(&start));
    stack.push(start);
    outside_air.insert(start);
    while let Some(cube) = stack.pop() {
        let neighbors: Vec<Cube> = cube
            .neighbors()
            .filter(|c| {
                (min_x..=max_x).contains(&c.0)
                    && (min_y..=max_y).contains(&c.1)
                    && (min_z..=max_z).contains(&c.2)
            })
            .filter(|c| !outside_air.contains(c))
            .filter(|c| !droplet.lava_cubes.contains(c))
            .collect();
        outside_air.extend(neighbors.iter().copied());
        stack.extend(neighbors);
    }

    // For every lava cube in the droplet, count its surfaces that border cubes in outside air.
    droplet
        .lava_cubes
        .iter()
        .flat_map(|c| c.neighbors().filter(|c| outside_air.contains(c)))
        .count()
}

struct Droplet {
    lava_cubes: HashSet<Cube>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Cube(i64, i64, i64);
impl Cube {
    fn neighbors(self) -> impl Iterator<Item = Self> {
        [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ]
        .into_iter()
        .map(move |d| Cube(self.0 + d.0, self.1 + d.1, self.2 + d.2))
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Cube> {
        all_consuming(terminated(separated_list1(line_ending, parse_cube), multispace0))(s)
            .unwrap()
            .1
    }

    fn parse_cube(s: &str) -> IResult<&str, Cube> {
        let (s, x) = i64(s)?;
        let (s, _) = char(',')(s)?;
        let (s, y) = i64(s)?;
        let (s, _) = char(',')(s)?;
        let (s, z) = i64(s)?;
        Ok((s, Cube(x, y, z)))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 64);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 58);
}
