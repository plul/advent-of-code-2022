//! Day 9: Rope Bridge
//!
//! https://adventofcode.com/2022/day/9

use std::cmp::max;
use std::collections::HashSet;
use std::ops::AddAssign;
use std::ops::Sub;

pub fn part_1(input: &str) -> usize {
    let moves = parser::parse(input);

    let mut head = Vector2D::default();
    let mut tail = head;

    let mut tail_visited = HashSet::<Vector2D>::new();
    tail_visited.insert(tail);

    for m in moves {
        for _ in 0..m.count {
            // Move head
            let head_movement = match m.direction {
                Direction::Right => (1, 0),
                Direction::Left => (-1, 0),
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
            };
            head += head_movement;

            // Move tail
            if head.chebyshev_distance(tail) > 1 {
                tail += (head - tail).clamp_x(-1, 1).clamp_y(-1, 1);
                tail_visited.insert(tail);
            }
        }
    }

    tail_visited.len()
}

pub fn part_2(input: &str) -> usize {
    parser::parse(input);
    Default::default()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default, Hash)]
struct Vector2D {
    /// Horizontal, positive in the right-direction.
    x: i32,

    /// Vertical, positive in the up-direction.
    y: i32,
}
impl Vector2D {
    fn chebyshev_distance(self, other: Self) -> i32 {
        let delta = self - other;
        max(delta.x.abs(), delta.y.abs())
    }
    fn clamp_x(mut self, min: i32, max: i32) -> Self {
        self.x = self.x.clamp(min, max);
        self
    }
    fn clamp_y(mut self, min: i32, max: i32) -> Self {
        self.y = self.y.clamp(min, max);
        self
    }
}
impl From<(i32, i32)> for Vector2D {
    fn from(value: (i32, i32)) -> Self {
        Vector2D {
            x: value.0,
            y: value.1,
        }
    }
}
impl<T> AddAssign<T> for Vector2D
where
    T: Into<Self>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T> Sub<T> for Vector2D
where
    T: Into<Self>,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self {
        let rhs = rhs.into();
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

struct Move {
    direction: Direction,
    count: u32,
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Move> {
        all_consuming(many0(main_parser))(s).unwrap().1
    }

    fn main_parser(s: &str) -> IResult<&str, Move> {
        let (s, (direction, count)) =
            terminated(separated_pair(one_of("RLUD"), char(' '), u32), line_ending)(s)?;
        let direction = match direction {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unreachable!(),
        };
        let m = Move { direction, count };
        Ok((s, m))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 13);
}

// #[test]
// fn part_2_example() {
//     assert_eq!(part_2(EXAMPLE), 0);
// }
