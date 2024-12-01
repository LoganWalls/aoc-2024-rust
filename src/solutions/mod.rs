pub mod day1;
use aoc::cli;

use anyhow::{anyhow, Context, Result};

macro_rules! timed {
    ($f:block) => {{
        let start = std::time::Instant::now();
        let result = $f;
        let stop = start.elapsed();
        println!("Took {stop:?}");
        result
    }};
}

pub trait Solution {
    fn part1(input: &str) -> Result<usize>;

    #[allow(unused_variables)]
    fn part2(input: &str) -> Result<usize> {
        unimplemented!();
    }

    fn run(config: &cli::Args) -> Result<()> {
        println!("Running Day {} Part {}", config.day, config.part);
        let input = std::fs::read_to_string(config.input_path()).with_context(|| {
            format!("missing input for day {} part {}", config.day, config.part)
        })?;
        let result = match config.part {
            1 => {
                timed!({ Self::part1(&input) })
            }
            2 => timed!({ Self::part2(&input) }),
            _ => Err(anyhow!("Part must be 1 or 2"))?,
        }?;
        println!("Answer: {result}");
        Ok(())
    }
}
