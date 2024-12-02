use std::collections::BinaryHeap;

use itertools::Itertools;

use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn input() -> &'static str {
        include_str!("../../inputs/day1.txt")
    }

    fn example_input() -> &'static str {
        include_str!("../../inputs/day1.example.txt")
    }

    fn part1(input: &str) -> anyhow::Result<usize> {
        let (left, right): (BinaryHeap<_>, BinaryHeap<_>) = input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("   ").expect("two items per line");
                (
                    a.parse::<i32>().expect("item is number"),
                    b.parse::<i32>().expect("item is number"),
                )
            })
            .unzip();
        Ok(left
            .into_sorted_vec()
            .into_iter()
            .zip(right.into_sorted_vec())
            .map(|(a, b)| (a - b).unsigned_abs() as usize)
            .sum())
    }

    fn part2(input: &str) -> anyhow::Result<usize> {
        let (left, right): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("   ").expect("two items per line");
                (
                    a.parse::<usize>().expect("item is number"),
                    b.parse::<usize>().expect("item is number"),
                )
            })
            .unzip();
        let counts = right.into_iter().counts();
        Ok(left
            .into_iter()
            .map(|x| x * counts.get(&x).unwrap_or(&0))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Day1::example_input();
        let result = Day1::part1(input).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn part2() {
        let input = Day1::example_input();
        let result = Day1::part2(input).unwrap();
        assert_eq!(result, 31);
    }
}
