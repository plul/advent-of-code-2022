//! Day 2: Rock Paper Scissors
//!
//! https://adventofcode.com/2022/day/2

pub fn part_1(input: &str) -> usize {
    let strategy_guide = parser::parse(input);

    fn choose(s: &Strategy) -> RockPaperScissors {
        match s.us {
            EncryptedStrategy::X => RockPaperScissors::Rock,
            EncryptedStrategy::Y => RockPaperScissors::Paper,
            EncryptedStrategy::Z => RockPaperScissors::Scissors,
        }
    }

    strategy_guide
        .strategies
        .iter()
        .map(|s| {
            let us = choose(s);
            us.score_shape() + us.outcome(&s.them).score()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let strategy_guide = parser::parse(input);

    fn choose(s: &Strategy) -> RockPaperScissors {
        let us = match s.us {
            EncryptedStrategy::X => match s.them {
                RockPaperScissors::Rock => RockPaperScissors::Scissors,
                RockPaperScissors::Paper => RockPaperScissors::Rock,
                RockPaperScissors::Scissors => RockPaperScissors::Paper,
            },
            EncryptedStrategy::Y => s.them,
            EncryptedStrategy::Z => match s.them {
                RockPaperScissors::Rock => RockPaperScissors::Paper,
                RockPaperScissors::Paper => RockPaperScissors::Scissors,
                RockPaperScissors::Scissors => RockPaperScissors::Rock,
            },
        };

        #[cfg(debug_assertions)]
        {
            let expected = match s.us {
                EncryptedStrategy::X => RoundOutcome::Loss,
                EncryptedStrategy::Y => RoundOutcome::Draw,
                EncryptedStrategy::Z => RoundOutcome::Win,
            };
            debug_assert_eq!(us.outcome(&s.them), expected);
        }

        us
    }

    strategy_guide
        .strategies
        .iter()
        .map(|s| {
            let us: RockPaperScissors = choose(s);
            us.score_shape() + us.outcome(&s.them).score()
        })
        .sum()
}

struct StrategyGuide {
    strategies: Vec<Strategy>,
}

struct Strategy {
    them: RockPaperScissors,
    us: EncryptedStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EncryptedStrategy {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}
impl RockPaperScissors {
    fn score_shape(&self) -> usize {
        match self {
            RockPaperScissors::Rock => 1,
            RockPaperScissors::Paper => 2,
            RockPaperScissors::Scissors => 3,
        }
    }

    fn outcome(&self, them: &Self) -> RoundOutcome {
        match (self, them) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => RoundOutcome::Draw,
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => RoundOutcome::Loss,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => RoundOutcome::Win,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => RoundOutcome::Win,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => RoundOutcome::Draw,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => RoundOutcome::Loss,
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => RoundOutcome::Loss,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => RoundOutcome::Win,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => RoundOutcome::Draw,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RoundOutcome {
    Win,
    Loss,
    Draw,
}
impl RoundOutcome {
    fn score(&self) -> usize {
        match self {
            RoundOutcome::Win => 6,
            RoundOutcome::Loss => 0,
            RoundOutcome::Draw => 3,
        }
    }
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> StrategyGuide {
        all_consuming(parse_strategy_guide)(s).unwrap().1
    }

    fn parse_strategy_guide(s: &str) -> IResult<&str, StrategyGuide> {
        let (s, strategies) = many0(parse_strategy)(s)?;
        let guide = StrategyGuide { strategies };
        Ok((s, guide))
    }

    fn parse_strategy(s: &str) -> IResult<&str, Strategy> {
        let (s, them) = parse_rock_paper_scissors_them(s)?;
        let (s, _) = nom::character::complete::char(' ')(s)?;
        let (s, us) = parse_encrypted_strategy(s)?;
        let (s, _) = line_ending(s)?;
        let strategy = Strategy { them, us };
        Ok((s, strategy))
    }

    fn parse_rock_paper_scissors_them(s: &str) -> IResult<&str, RockPaperScissors> {
        let (s, them) = one_of("ABC")(s)?;
        let rock_paper_scissors = match them {
            'A' => RockPaperScissors::Rock,
            'B' => RockPaperScissors::Paper,
            'C' => RockPaperScissors::Scissors,
            _ => unreachable!(),
        };
        Ok((s, rock_paper_scissors))
    }

    fn parse_encrypted_strategy(s: &str) -> IResult<&str, EncryptedStrategy> {
        let (s, encrypted) = one_of("XYZ")(s)?;
        let encrypted_strategy = match encrypted {
            'X' => EncryptedStrategy::X,
            'Y' => EncryptedStrategy::Y,
            'Z' => EncryptedStrategy::Z,
            _ => unreachable!(),
        };
        Ok((s, encrypted_strategy))
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
A Y
B X
C Z
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 15);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 12);
}
