//! Day 14: Regolith Reservoir
//!
//! https://adventofcode.com/2022/day/14

use crate::lib::vector_2d::Vector2D;
use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    let paths = parser::parse(input);

    // Find largest y value of rock so we know when a corn of sand would be falling endlessly.
    let max_y_of_rock = paths.iter().flat_map(|p| p.iter()).map(|v| v.y).max().unwrap();

    // Build world
    let mut world: HashMap<Vector2D<i64>, Element> = HashMap::new();
    for path in paths {
        let mut path_iter = path.into_iter();
        let mut current_point = path_iter.next().unwrap();
        world.insert(current_point, Element::Rock);
        for next_point_in_path in path_iter {
            while current_point != next_point_in_path {
                let diff = next_point_in_path - current_point;
                let step = diff.clamp_x(-1, 1).clamp_y(-1, 1);
                current_point += step;
                world.insert(current_point, Element::Rock);
            }
        }
    }

    // Simulate falling sand
    let spawn_point: Vector2D<i64> = Vector2D::from((500, 0));
    let mut units_of_sand_come_to_rest = 0;
    'main_loop: loop {
        debug_assert!(!world.contains_key(&spawn_point), "spawn point should be air");
        let mut current_point = spawn_point;

        loop {
            if current_point.y > max_y_of_rock {
                // End of simulation
                break 'main_loop;
            }

            if !world.contains_key(&(current_point + (0, 1))) {
                current_point += (0, 1);
            } else if !world.contains_key(&(current_point + (-1, 1))) {
                current_point += (-1, 1);
            } else if !world.contains_key(&(current_point + (1, 1))) {
                current_point += (1, 1);
            } else {
                // Deposit here.
                world.insert(current_point, Element::Sand);
                units_of_sand_come_to_rest += 1;
                break;
            }
        }
    }

    units_of_sand_come_to_rest
}

pub fn part_2(input: &str) -> usize {
    let paths = parser::parse(input);

    // Find largest y value of rock so we know when a corn of sand would be falling endlessly.
    let max_y_of_rock = paths.iter().flat_map(|p| p.iter()).map(|v| v.y).max().unwrap();

    let floor = max_y_of_rock + 2;

    // Build world
    let mut world: HashMap<Vector2D<i64>, Element> = HashMap::new();
    for path in paths {
        let mut path_iter = path.into_iter();
        let mut current_point = path_iter.next().unwrap();
        world.insert(current_point, Element::Rock);
        for next_point_in_path in path_iter {
            while current_point != next_point_in_path {
                let diff = next_point_in_path - current_point;
                let step = diff.clamp_x(-1, 1).clamp_y(-1, 1);
                current_point += step;
                world.insert(current_point, Element::Rock);
            }
        }
    }

    // Simulate falling sand
    let spawn_point: Vector2D<i64> = Vector2D::from((500, 0));
    let mut units_of_sand_come_to_rest = 0;
    loop {
        if world.contains_key(&spawn_point) {
            // End of simulation
            break;
        }

        let mut current_point = spawn_point;

        loop {
            if current_point.y + 1 >= floor {
                // Deposit here.
                world.insert(current_point, Element::Sand);
                units_of_sand_come_to_rest += 1;
                break;
            }

            if !world.contains_key(&(current_point + (0, 1))) {
                current_point += (0, 1);
            } else if !world.contains_key(&(current_point + (-1, 1))) {
                current_point += (-1, 1);
            } else if !world.contains_key(&(current_point + (1, 1))) {
                current_point += (1, 1);
            } else {
                // Deposit here.
                world.insert(current_point, Element::Sand);
                units_of_sand_come_to_rest += 1;
                break;
            }
        }
    }

    units_of_sand_come_to_rest
}

type PathOfRock = Vec<Vector2D<i64>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Rock,
    Sand,
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<PathOfRock> {
        all_consuming(many0(parse_path_of_rock_line))(s).unwrap().1
    }

    fn parse_path_of_rock_line(s: &str) -> IResult<&str, PathOfRock> {
        terminated(separated_list1(tag(" -> "), parse_coordinate), line_ending)(s)
    }

    fn parse_coordinate(s: &str) -> IResult<&str, Vector2D<i64>> {
        map(separated_pair(i64, char(','), i64), Vector2D::from)(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 24);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 93);
}
