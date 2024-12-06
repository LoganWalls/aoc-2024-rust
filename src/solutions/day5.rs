use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use super::Solution;

pub struct Day5;

fn sort_pages(rules: &HashMap<usize, Vec<usize>>, a: &usize, b: &usize) -> Ordering {
    if let Some(must_come_after) = rules.get(a) {
        if must_come_after.contains(b) {
            return Ordering::Less;
        }
    } else if let Some(must_come_after) = rules.get(b) {
        if must_come_after.contains(a) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

impl Solution for Day5 {
    fn input() -> &'static str {
        include_str!("../../inputs/day5.txt")
    }

    fn example_input() -> &'static str {
        include_str!("../../inputs/day5.example.txt")
    }

    fn part1(input: &str) -> anyhow::Result<usize> {
        let (rules, updates) = input.split_once("\n\n").expect("rules and updates");
        let rules = rules
            .lines()
            .map(|line| -> (usize, usize) {
                let (left, right) = line.split_once("|").expect("each rule has two parts");
                (
                    left.parse().expect("valid page number"),
                    right.parse().expect("valid page number"),
                )
            })
            .into_group_map();

        Ok(updates
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<usize>().expect("valid page number"))
                    .collect::<Vec<_>>()
            })
            .filter(|pages| pages.is_sorted_by(|a, b| sort_pages(&rules, a, b).is_lt()))
            .map(|pages| pages[pages.len() / 2])
            .sum())
    }

    fn part2(input: &str) -> anyhow::Result<usize> {
        let (rules, updates) = input.split_once("\n\n").expect("rules and updates");
        let rules = rules
            .lines()
            .map(|line| -> (usize, usize) {
                let (left, right) = line.split_once("|").expect("each rule has two parts");
                (
                    left.parse().expect("valid page number"),
                    right.parse().expect("valid page number"),
                )
            })
            .into_group_map();

        Ok(updates
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<usize>().expect("valid page number"))
                    .collect::<Vec<_>>()
            })
            .filter(|pages| !pages.is_sorted_by(|a, b| sort_pages(&rules, a, b).is_lt()))
            .map(|pages| {
                let half = pages.len() / 2;
                pages
                    .into_iter()
                    .sorted_by(|a, b| sort_pages(&rules, a, b))
                    .nth(half)
                    .expect("valid middle element")
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = Day5::example_input();
        let result = Day5::part1(input).unwrap();
        assert_eq!(result, 143);
    }

    #[test]
    fn part2() {
        let input = Day5::example_input();
        let result = Day5::part2(input).unwrap();
        assert_eq!(result, 123);
    }
}
