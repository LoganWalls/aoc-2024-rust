use itertools::Itertools;

use super::Solution;

pub struct Day7;

#[derive(Debug, Clone)]
enum Op {
    Add,
    Multiply,
    Concat,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a.saturating_add(b),
            Self::Multiply => a.saturating_mul(b),
            Self::Concat => {
                let decimals = b.ilog10() + 1;
                (a.saturating_mul(10u64.saturating_pow(decimals))).saturating_add(b)
            }
        }
    }
}

fn solution(input: &str, ops: &[Op]) -> anyhow::Result<usize> {
    Ok(input
        .lines()
        .filter_map(|line| {
            let (test_value, numbers) = line.split_once(": ").expect("standard line format");
            let test_value: u64 = test_value.parse().expect("valid number");
            let numbers: Vec<u64> = numbers
                .split(" ")
                .map(|v| v.parse().expect("valid number"))
                .collect();
            for ops in (1..numbers.len())
                .map(|_| ops.iter())
                .multi_cartesian_product()
            {
                let result = numbers[1..]
                    .iter()
                    .cloned()
                    .zip(ops)
                    .fold(numbers[0], |left, (right, op)| op.apply(left, right));
                if result == test_value {
                    return Some(test_value);
                }
            }
            None
        })
        .sum::<u64>() as usize)
}

impl Solution for Day7 {
    fn input() -> &'static str {
        include_str!("../../inputs/day7.txt")
    }

    fn example_input() -> &'static str {
        include_str!("../../inputs/day7.example.txt")
    }

    fn part1(input: &str) -> anyhow::Result<usize> {
        solution(input, &[Op::Add, Op::Multiply])
    }

    fn part2(input: &str) -> anyhow::Result<usize> {
        solution(input, &[Op::Add, Op::Multiply, Op::Concat])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Day7::example_input();
        let result = Day7::part1(input).unwrap();
        assert_eq!(result, 3749);
    }

    #[test]
    fn part2() {
        let input = Day7::example_input();
        let result = Day7::part2(input).unwrap();
        assert_eq!(result, 11387);
    }
}

// 4342 too high
// 3835 wrong, probably too high?
//
// Barriers:
// (3, 6)
// (6, 7)
// (7, 7)
// (1, 8)
// (3, 8)
// (7, 9)
//
//
// (6, 6) is the extra one / wrong one.
