//! Day 10: Cathode-Ray Tube
//!
//! https://adventofcode.com/2022/day/10

use std::fmt::Display;

pub fn part_1(input: &str) -> i64 {
    let instructions = parser::parse(input);
    let mut cpu = CpuEmulator::default();
    instructions
        .into_iter()
        .flat_map(|ins| cpu.feed(ins))
        .enumerate()
        .map(|(cycle, tick)| (cycle + 1, tick))
        .skip(19)
        .step_by(40)
        .map(|(cycle, tick)| signal_strength(cycle, tick.state_before.register))
        .sum()
}

pub fn part_2(input: &str) -> CrtImage {
    let instructions = parser::parse(input);
    let mut cpu = CpuEmulator::default();
    instructions
        .into_iter()
        .flat_map(|ins| cpu.feed(ins))
        .enumerate()
        .map(|(cycle, tick)| (cycle + 1, tick))
        .map(|(cycle, tick)| {
            let col = ((cycle - 1) % 40) as i64;
            let lit = col.abs_diff(tick.state_before.register) <= 1;
            if lit { '#' } else { '.' }
        })
        .collect()
}

fn signal_strength(cycle: usize, register: i64) -> i64 {
    cycle as i64 * register
}

pub struct CrtImage {
    rows: Vec<String>,
}
impl FromIterator<char> for CrtImage {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut rows = vec![];
        loop {
            let row: String = iter.by_ref().take(40).collect();
            if row.is_empty() {
                break;
            }
            rows.push(row);
        }
        CrtImage { rows }
    }
}
impl Display for CrtImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for row in &self.rows {
            writeln!(f, "{row}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
struct CpuEmulator {
    state: State,
}
impl CpuEmulator {
    fn feed(&mut self, instruction: Instruction) -> impl IntoIterator<Item = Tick> {
        match instruction {
            Instruction::Noop => vec![Tick {
                state_before: self.state,
                state_after: self.state,
            }],
            Instruction::Addx(x) => {
                let state_before = self.state;
                self.state.register += x;
                vec![
                    Tick {
                        state_before,
                        state_after: state_before,
                    },
                    Tick {
                        state_before,
                        state_after: self.state,
                    },
                ]
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct State {
    register: i64,
}
impl Default for State {
    fn default() -> Self {
        Self { register: 1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Tick {
    /// Before (and during) cycle.
    state_before: State,

    /// After cycle.
    state_after: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Noop,
    Addx(i64),
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Vec<Instruction> {
        all_consuming(many0(parse_instruction))(s).unwrap().1
    }

    fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
        let noop = value(Instruction::Noop, tag("noop"));
        let addx = map(preceded(tag("addx "), i64), Instruction::Addx);
        terminated(alt((noop, addx)), line_ending)(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 13140);
}
