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
        Ok(1)
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
}
