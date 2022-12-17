//! Day 17: Pyroclastic Flow
//!
//! https://adventofcode.com/2022/day/17

use crate::lib::vector_2d::Vector2D;
use std::collections::VecDeque;
use std::fmt::Display;

pub fn part_1(input: &str) -> usize {
    let jets = parser::parse(input);
    let cave = simulate(jets, 2022);
    cave.lines_cleared_tetris_style + cave.rows.len() - 1
}

pub fn part_2(input: &str) -> usize {
    let jets = parser::parse(input);
    let cave = simulate(jets, 1_000_000_000_000);
    cave.lines_cleared_tetris_style + cave.rows.len() - 1
}

fn simulate(jets: Vec<Jet>, limit: usize) -> Cave {
    let mut jet_pattern = jet_pattern(&jets);
    let rock_pattern = rock_pattern();

    let floor = Row([true; 7]);
    let mut cave = Cave {
        lines_cleared_tetris_style: 0,
        rows: VecDeque::from([floor]),
    };

    for (i, rock_shape) in rock_pattern.take(limit).enumerate() {
        if (i / 10_000_000) * 10_000_000 == i {
            let progress = i * 100 / 1_514_285_714_288;
            println!("Simulating rock #{i}. Estimated progress: {progress} %");
        }

        let mut falling_rock = FallingRock {
            coord: Vector2D::from((cave.rows.len() as i64 + 3, 2)),
            rock_shape,
        };

        loop {
            let jet = jet_pattern.next().unwrap();
            let movement: Vector2D<i64> = match jet {
                Jet::Left => (0, -1),
                Jet::Right => (0, 1),
            }
            .into();
            if falling_rock.can_move(movement, &cave) {
                falling_rock.apply_movement(movement);
            }

            let down = (-1, 0).into();
            if falling_rock.can_move(down, &cave) {
                falling_rock.apply_movement(down);
            } else {
                falling_rock.come_to_rest(&mut cave);
                break;
            }
        }
    }

    cave
}

fn jet_pattern(jets: &[Jet]) -> impl Iterator<Item = &Jet> {
    std::iter::repeat(jets).flat_map(|jets| jets.iter())
}

fn rock_pattern() -> impl Iterator<Item = RockShape> {
    // Rock shaped like -
    let rock_1 = RockShape(
        [(0, 0), (0, 1), (0, 2), (0, 3)]
            .into_iter()
            .map(Vector2D::from)
            .collect(),
    );

    // Rock shaped like +
    let rock_2 = RockShape(
        [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]
            .into_iter()
            .map(Vector2D::from)
            .collect(),
    );

    // Rock shaped like an angle
    let rock_3 = RockShape(
        [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]
            .into_iter()
            .map(Vector2D::from)
            .collect(),
    );

    // Rock shaped like |
    let rock_4 = RockShape(
        [(0, 0), (1, 0), (2, 0), (3, 0)]
            .into_iter()
            .map(Vector2D::from)
            .collect(),
    );

    // Rock shaped like box
    let rock_5 = RockShape(
        [(0, 0), (0, 1), (1, 0), (1, 1)]
            .into_iter()
            .map(Vector2D::from)
            .collect(),
    );

    let rocks = vec![rock_1, rock_2, rock_3, rock_4, rock_5];

    std::iter::repeat(rocks).flat_map(|rocks| rocks.into_iter())
}

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug)]
struct FallingRock {
    coord: Vector2D<i64>,
    rock_shape: RockShape,
}
impl FallingRock {
    fn can_move(&self, movement: Vector2D<i64>, cave: &Cave) -> bool {
        !self
            .rock_shape
            .0
            .iter()
            .map(|&part_coord| part_coord + movement + self.coord)
            .any(|coord| {
                // Check if enters walls
                if coord.y < 0 || coord.y >= 7 {
                    return true;
                }

                // Check if enters other rock or floor
                if cave
                    .rows
                    .get(coord.x as usize)
                    .map(|row| row.0[coord.y as usize])
                    .unwrap_or(false)
                {
                    return true;
                }

                false
            })
    }

    fn apply_movement(&mut self, movement: Vector2D<i64>) {
        self.coord += movement;
    }

    fn come_to_rest(self, cave: &mut Cave) {
        let mut full_line = None;

        for part_of_rock in self
            .rock_shape
            .0
            .into_iter()
            .map(|part_coord| part_coord + self.coord)
        {
            let (row, col) = (part_of_rock.x, part_of_rock.y);
            while row >= cave.rows.len() as i64 {
                cave.rows.push_back(Row([false; 7]));
            }
            cave.rows[row as usize].0[col as usize] = true;

            if cave.rows[row as usize].0 == [true; 7] {
                full_line = Some(row);
            }
        }

        // Clear full rows
        if let Some(row) = full_line {
            let relevant_rows = cave.rows.split_off(row as usize);
            cave.lines_cleared_tetris_style += cave.rows.len();
            cave.rows = relevant_rows;
        }
    }
}

/// Rocks in terms of (row, col), where lower left point is (0,0).
#[derive(Clone, Debug)]
struct RockShape(Vec<Vector2D<i64>>);

struct Cave {
    lines_cleared_tetris_style: usize,
    rows: VecDeque<Row>,
}
impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter().rev() {
            write!(f, "|")?;
            for col in row.0 {
                if col {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Row([bool; 7]);

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Jet> {
        all_consuming(terminated(many1(parse_jet), multispace0))(s)
            .unwrap()
            .1
    }

    fn parse_jet(s: &str) -> IResult<&str, Jet> {
        let left = value(Jet::Left, char('<'));
        let right = value(Jet::Right, char('>'));
        alt((left, right))(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 3068);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 1_514_285_714_288);
}
