use itertools::{izip, Itertools};
use std::iter::repeat_n;

use super::Solution;

pub struct Day4;

macro_rules! matches_xmas {
    ($input: expr) => {
        matches!($input, (b'X', b'M', b'A', b'S') | (b'S', b'A', b'M', b'X'))
    };
}

impl Solution for Day4 {
    fn input() -> &'static str {
        include_str!("../../inputs/day4.txt")
    }

    fn example_input() -> &'static str {
        include_str!("../../inputs/day4.example.txt")
    }

    fn part1(input: &str) -> anyhow::Result<usize> {
        let line_length = input.find("\n").expect("at least one line");
        type Column = (u8, u8, u8, u8);
        Ok(input
            .lines()
            .chain(repeat_n(".".repeat(line_length).as_str(), 3))
            .tuple_windows()
            .fold(0, |acc: usize, lines: (&str, &str, &str, &str)| {
                acc + izip!(
                    lines.0.bytes().chain(repeat_n(b'.', 3)),
                    lines.1.bytes().chain(repeat_n(b'.', 3)),
                    lines.2.bytes().chain(repeat_n(b'.', 3)),
                    lines.3.bytes().chain(repeat_n(b'.', 3)),
                )
                .tuple_windows::<(Column, Column, Column, Column)>()
                .map(|columns| {
                    let horizontal =
                        matches_xmas!((columns.0 .0, columns.1 .0, columns.2 .0, columns.3 .0));
                    let vertical = matches_xmas!(columns.0);
                    let right_diagonal =
                        matches_xmas!((columns.0 .0, columns.1 .1, columns.2 .2, columns.3 .3));
                    let left_diagonal =
                        matches_xmas!((columns.3 .0, columns.2 .1, columns.1 .2, columns.0 .3));
                    horizontal as usize
                        + vertical as usize
                        + right_diagonal as usize
                        + left_diagonal as usize
                })
                .sum::<usize>()
            }))
    }

    fn part2(input: &str) -> anyhow::Result<usize> {
        type Column = (u8, u8, u8);
        Ok(input
            .lines()
            .tuple_windows()
            .fold(0, |acc: usize, lines: (&str, &str, &str)| {
                acc + izip!(lines.0.bytes(), lines.1.bytes(), lines.2.bytes())
                    .tuple_windows::<(Column, Column, Column)>()
                    .filter(|columns| {
                        columns.1 .1 == b'A'
                            && matches!(
                                (columns.0, columns.2),
                                ((b'M', _, b'M'), (b'S', _, b'S'))
                                    | ((b'S', _, b'S'), (b'M', _, b'M'))
                                    | ((b'M', _, b'S'), (b'M', _, b'S'))
                                    | ((b'S', _, b'M'), (b'S', _, b'M'))
                            )
                    })
                    .count()
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Day4::example_input();
        let result = Day4::part1(input).unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn part2() {
        let input = Day4::example_input();
        let result = Day4::part2(input).unwrap();
        assert_eq!(result, 9);
    }
}
