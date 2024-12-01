use anyhow::{anyhow, Context, Result};
use clap::Parser;

mod solutions;
use aoc::cli;

use self::solutions::Solution;

fn main() -> Result<()> {
    let config = cli::Args::parse();
    match config.command {
        cli::Command::Run => {
            match config.day {
                1 => solutions::day1::Day1::run(&config),
                _ => Err(anyhow!("Day {} not implemented", config.day)),
            }?;
        }
        cli::Command::Fetch => {
            let problem = ureq::get(&format!(
                "https://adventofcode.com/2024/day/{}/input",
                config.day
            ))
            .set("cookie", env!("AOC_KEY"))
            .call()?
            .into_string()?;
            let path = config.input_path();
            std::fs::write(&path, problem)
                .with_context(|| format!("Could not write input to: {path}"))?;
        }
    }
    Ok(())
}
