//! Day 9: Rope Bridge
//!
//! https://adventofcode.com/2022/day/9

use std::cmp::max;
use std::collections::HashSet;
use std::ops::AddAssign;
use std::ops::Sub;

pub fn part_1(input: &str) -> usize {
    let moves = parser::parse(input);
    let rope = Rope::new(2);
    count_positions_visited_by_tail(rope, moves)
}

pub fn part_2(input: &str) -> usize {
    let moves = parser::parse(input);
    let rope = Rope::new(10);
    count_positions_visited_by_tail(rope, moves)
}

fn count_positions_visited_by_tail(mut rope: Rope, moves: Vec<Move>) -> usize {
    let mut tail_visited = HashSet::<Vector2D>::new();
    tail_visited.insert(rope.tail());

    for m in moves {
        for _ in 0..m.count {
            rope.move_rope(m);
            tail_visited.insert(rope.tail());
        }
    }

    tail_visited.len()
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Vector2D>,
}
impl Rope {
    fn new(knots: usize) -> Self {
        Self {
            knots: vec![Vector2D::default(); knots],
        }
    }

    fn move_rope(&mut self, head_movement: Move) {
        // Move head knot
        let head_movement = match head_movement.direction {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };
        *self.knots.first_mut().unwrap() += head_movement;

        // Move rest of knots
        for idx in 0.. {
            let knot_a = self.knots.get(idx).copied().unwrap();
            let Some(knot_b) = self.knots.get_mut(idx+1) else { break };

            if knot_a.chebyshev_distance(*knot_b) > 1 {
                *knot_b += (knot_a - *knot_b).clamp_x(-1, 1).clamp_y(-1, 1);
            }
        }
    }

    fn tail(&self) -> Vector2D {
        *self.knots.last().unwrap()
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    direction: Direction,
    count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
static EXAMPLE_PART_2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 13);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE_PART_2), 36);
}
