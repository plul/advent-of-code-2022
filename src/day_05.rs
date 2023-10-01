//! Day 5: Supply Stacks
//!
//! https://adventofcode.com/2022/day/5

pub fn part_1(input: &str) -> String {
    let Input {
        mut stacks,
        move_instructions,
    } = parser::parse(input);

    for ins in move_instructions {
        for _ in 0..ins.count {
            let item = stacks[ins.from - 1].pop().unwrap();
            stacks[ins.to - 1].push(item);
        }
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

pub fn part_2(input: &str) -> String {
    let Input {
        mut stacks,
        move_instructions,
    } = parser::parse(input);

    for ins in move_instructions {
        let v = &mut stacks[ins.from - 1];
        let to_move = v.split_off(v.len() - ins.count);
        stacks[ins.to - 1].extend(to_move);
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

#[derive(Debug)]
struct Input {
    stacks: Vec<Vec<char>>,
    move_instructions: Vec<MoveInstruction>,
}

#[derive(Debug)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize,
}

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> Input {
        all_consuming(main_parser)(s).unwrap().1
    }

    fn main_parser(s: &str) -> IResult<&str, Input> {
        let (s, stack_lines) = many1(parse_stack_line)(s)?;
        let (s, stack_numbers) = parse_stack_number_line(s)?;
        let (s, _) = line_ending(s)?;
        let (s, move_instructions) = many1(parse_move_instruction_line)(s)?;

        assert_eq!(stack_lines.iter().map(|s| s.len()).max().unwrap(), stack_numbers.len());

        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_numbers.len()];
        for stack_line in stack_lines.into_iter().rev() {
            for (stack, item) in stacks.iter_mut().zip(stack_line.into_iter()) {
                if let Some(item) = item {
                    stack.push(item);
                }
            }
        }

        let input = Input { stacks, move_instructions };

        Ok((s, input))
    }

    fn parse_stack_line(s: &str) -> IResult<&str, Vec<Option<char>>> {
        let capital_letter = satisfy(|c| {
            let range = ('A' as usize)..=('Z' as usize);
            range.contains(&(c as usize))
        });
        let something = map(delimited(char('['), capital_letter, char(']')), Some);
        let nothing = map(tag("   "), |_| None);
        let something_or_nothing = alt((something, nothing));
        terminated(separated_list0(char(' '), something_or_nothing), line_ending)(s)
    }
    #[test]
    fn test_parse_stack_line() {
        assert_eq!(parse_stack_line("    [D]\n").unwrap().1, vec![None, Some('D')]);
        assert_eq!(parse_stack_line("[N] [C]\n").unwrap().1, vec![Some('N'), Some('C')]);
    }

    fn parse_stack_number_line(s: &str) -> IResult<&str, Vec<u32>> {
        let stack_numbers = preceded(
            char(' '),
            verify(separated_list1(space1, u32), |list: &Vec<u32>| {
                list.iter().copied().eq(1..=(list.len() as u32))
            }),
        );
        terminated(stack_numbers, line_ending)(s)
    }
    #[test]
    fn test_parse_stack_number_line() {
        parse_stack_number_line(" 1   2   3\n").unwrap();
    }

    fn parse_move_instruction_line(s: &str) -> IResult<&str, MoveInstruction> {
        let (s, _) = tag("move")(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, count) = u32(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, _) = tag("from")(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, from) = u32(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, _) = tag("to")(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, to) = u32(s)?;
        let (s, _) = line_ending(s)?;

        let move_instruction = MoveInstruction {
            count: count as usize,
            from: from as usize,
            to: to as usize,
        };

        Ok((s, move_instruction))
    }
}

#[cfg(test)]
static EXAMPLE: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), "CMZ");
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), "MCD");
}
