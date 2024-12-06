pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
use aoc::cli;

use anyhow::{anyhow, Result};

macro_rules! timed {
    ($f:block) => {{
        let start = std::time::Instant::now();
        let result = $f;
        let stop = start.elapsed();
        println!("Took: {stop:?}");
        result
    }};
}

pub trait Solution {
    fn input() -> &'static str;

    #[allow(dead_code)]
    fn example_input() -> &'static str;

    fn part1(input: &str) -> Result<usize>;

    #[allow(unused_variables)]
    fn part2(input: &str) -> Result<usize> {
        unimplemented!();
    }

    fn run(config: &cli::Config) -> Result<()> {
        println!("Running Day {} Part {}", config.day, config.part);
        let input = Self::input();
        let result = match config.part {
            1 => {
                timed!({ Self::part1(input) })
            }
            2 => timed!({ Self::part2(input) }),
            _ => Err(anyhow!("Part must be 1 or 2"))?,
        }?;
        println!("Answer: {result}");
        Ok(())
    }
}
