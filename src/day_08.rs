//! Day 8: Treetop Tree House
//!
//! https://adventofcode.com/2022/day/8

pub fn part_1(input: &str) -> usize {
    let patch_of_tall_trees = parser::parse(input);

    let tree_iter = patch_of_tall_trees.iter().enumerate().flat_map(|(row_idx, row)| {
        row.iter()
            .enumerate()
            .map(move |(col_idx, tree)| (row_idx, col_idx, tree))
    });

    tree_iter
        .filter(|(row_idx, col_idx, _tree)| is_visible(&patch_of_tall_trees, *row_idx, *col_idx))
        .count()
}

pub fn part_2(input: &str) -> usize {
    let patch_of_tall_trees = parser::parse(input);

    let tree_iter = patch_of_tall_trees.iter().enumerate().flat_map(|(row_idx, row)| {
        row.iter()
            .enumerate()
            .map(move |(col_idx, tree)| (row_idx, col_idx, tree))
    });

    tree_iter
        .map(|(row_idx, col_idx, _tree)| scenic_score(&patch_of_tall_trees, row_idx, col_idx))
        .max()
        .unwrap()
}

fn is_visible(patch_of_tall_trees: &PatchOfTallTrees, row_idx: usize, col_idx: usize) -> bool {
    let tree = patch_of_tall_trees[row_idx][col_idx];

    let row = patch_of_tall_trees[row_idx].iter();
    let visible_from_left = row.take(col_idx).max().map(|&m| m < tree).unwrap_or(true);

    let row = patch_of_tall_trees[row_idx].iter();
    let visible_from_right = row.skip(col_idx + 1).max().map(|&m| m < tree).unwrap_or(true);

    let column = patch_of_tall_trees.iter().map(|row| row[col_idx]);
    let visible_from_top = column.take(row_idx).max().map(|m| m < tree).unwrap_or(true);

    let column = patch_of_tall_trees.iter().map(|row| row[col_idx]);
    let visible_from_bottom = column.skip(row_idx + 1).max().map(|m| m < tree).unwrap_or(true);

    visible_from_left || visible_from_right || visible_from_top || visible_from_bottom
}

fn scenic_score(patch_of_tall_trees: &PatchOfTallTrees, row_idx: usize, col_idx: usize) -> usize {
    let tree_top_house = patch_of_tall_trees[row_idx][col_idx];

    // This would be easier if iterators had a take_until in addition to take_while that would include the last element
    // https://github.com/rust-lang/rust/issues/62208

    let mut view_is_blocked: bool;

    view_is_blocked = false;
    let viewing_distance_left = patch_of_tall_trees[row_idx]
        .iter()
        .take(col_idx)
        .rev()
        .take_while(|&tree| {
            if view_is_blocked {
                return false;
            }

            if *tree >= tree_top_house {
                view_is_blocked = true;
            }

            true
        })
        .count();

    view_is_blocked = false;
    let viewing_distance_right = patch_of_tall_trees[row_idx]
        .iter()
        .skip(col_idx + 1)
        .take_while(|&tree| {
            if view_is_blocked {
                return false;
            }

            if *tree >= tree_top_house {
                view_is_blocked = true;
            }

            true
        })
        .count();

    view_is_blocked = false;
    let viewing_distance_up = patch_of_tall_trees
        .iter()
        .map(|row| row[col_idx])
        .take(row_idx)
        .rev()
        .take_while(|&tree| {
            if view_is_blocked {
                return false;
            }

            if tree >= tree_top_house {
                view_is_blocked = true;
            }

            true
        })
        .count();

    view_is_blocked = false;
    let viewing_distance_down = patch_of_tall_trees
        .iter()
        .map(|row| row[col_idx])
        .skip(row_idx + 1)
        .take_while(|&tree| {
            if view_is_blocked {
                return false;
            }

            if tree >= tree_top_house {
                view_is_blocked = true;
            }

            true
        })
        .count();

    viewing_distance_left * viewing_distance_right * viewing_distance_up * viewing_distance_down
}

type PatchOfTallTrees = Vec<Vec<u8>>;

mod parser {
    use super::*;
    use crate::nom_complete::*;

    pub(super) fn parse(s: &str) -> PatchOfTallTrees {
        all_consuming(main_parser)(s).unwrap().1
    }

    fn main_parser(s: &str) -> IResult<&str, PatchOfTallTrees> {
        let digit = map(satisfy(|c| c.is_ascii_digit()), |c| c.to_digit(10).unwrap() as u8);
        let parse_line = terminated(many1(digit), line_ending);
        many1(parse_line)(s)
    }
}

#[cfg(test)]
static EXAMPLE: &str = "\
30373
25512
65332
33549
35390
";

#[test]
fn part_1_example() {
    assert_eq!(part_1(EXAMPLE), 21);
}

#[test]
fn part_2_example() {
    assert_eq!(part_2(EXAMPLE), 8);
}
