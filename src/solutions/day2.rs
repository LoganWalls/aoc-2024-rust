use std::iter::once;

use itertools::Itertools;

use super::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn input() -> &'static str {
        include_str!("../../inputs/day2.txt")
    }

    fn example_input() -> &'static str {
        include_str!("../../inputs/day2.example.txt")
    }

    fn part1(input: &str) -> anyhow::Result<usize> {
        Ok(input
            .lines()
            .map(|line| -> usize {
                line.split(" ")
                    .map(|x| x.parse::<i32>().expect("item is number"))
                    .tuple_windows()
                    .map(|(a, b)| a - b)
                    .tuple_windows()
                    .all(|(a, b)| {
                        (1..=3).contains(&a.abs())
                            && (1..=3).contains(&b.abs())
                            && (a.signum() == b.signum())
                    })
                    .into()
            })
            .sum())
    }

    fn part2(input: &str) -> anyhow::Result<usize> {
        Ok(input
            .lines()
            .filter(|line| {
                let levels: Vec<_> = line
                    .split(" ")
                    .map(|x| x.parse::<i32>().expect("item is number"))
                    .collect();
                once(levels.iter().collect())
                    .chain(levels.iter().combinations(levels.len() - 1))
                    .any(|level_subset| {
                        level_subset
                            .into_iter()
                            .tuple_windows()
                            .map(|(a, b)| a - b)
                            .tuple_windows()
                            .all(|(a, b): (i32, i32)| {
                                (1..=3).contains(&a.abs())
                                    && (1..=3).contains(&b.abs())
                                    && (a.signum() == b.signum())
                            })
                    })
            })
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Day2::example_input();
        let result = Day2::part1(input).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn part2() {
        let input = Day2::example_input();
        let result = Day2::part2(input).unwrap();
        assert_eq!(result, 4);
    }
}
