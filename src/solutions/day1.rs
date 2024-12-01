use super::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1(input: &str) -> anyhow::Result<usize> {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = Day1::part1(input).unwrap();
        assert_eq!(result, 11);
    }
}
