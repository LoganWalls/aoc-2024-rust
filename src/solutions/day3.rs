use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while1};
use nom::character::complete::char;
use nom::combinator::{map, opt, rest};
use nom::multi::fold_many1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

use super::Solution;

pub struct Day3;

fn valid_body(input: &str) -> IResult<&str, (usize, usize)> {
    map(
        tuple((
            nom::character::complete::u32,
            char(','),
            nom::character::complete::u32,
            char(')'),
        )),
        |(a, _, b, _)| (a as usize, b as usize),
    )(input)
}

fn instruction(input: &str) -> IResult<&str, Option<(usize, usize)>> {
    preceded(
        tuple((
            take_until("mul("),
            tag::<&str, &str, nom::error::Error<&str>>("mul("),
        )),
        opt(valid_body),
    )(input)
}

fn enabled_segment(input: &str) -> IResult<&str, &str> {
    alt((
        terminated(
            take_until("don't()"),
            opt(tuple((
                tag("don't()"),
                alt((terminated(take_until("do()"), tag("do()")), rest)),
            ))),
        ),
        // Require at least one character so we don't match empty string
        take_while1(|_| true),
    ))(input)
}

impl Solution for Day3 {
    fn input() -> &'static str {
        include_str!("../../inputs/day3.txt")
    }

    fn example_input() -> &'static str {
        include_str!("../../inputs/day3.example.txt")
    }

    fn part1(input: &str) -> anyhow::Result<usize> {
        Ok(fold_many1(
            instruction,
            || 0,
            |acc, instr| {
                if let Some((a, b)) = instr {
                    acc + a * b
                } else {
                    acc
                }
            },
        )(input)
        .expect("at least one instruction")
        .1)
    }

    fn part2(input: &str) -> anyhow::Result<usize> {
        Ok(fold_many1(
            map(enabled_segment, Self::part1),
            || 0,
            |acc, segment_sum| acc + segment_sum.expect("can parse segment"),
        )(input)
        .expect("at least one segment")
        .1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Day3::example_input();
        let result = Day3::part1(input).unwrap();
        assert_eq!(result, 161);
    }

    #[test]
    fn part2() {
        // Example is different from part 1
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = Day3::part2(input).unwrap();
        assert_eq!(result, 48);
    }
}
