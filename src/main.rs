use anyhow::{anyhow, Result};
use clap::Parser;

mod solutions;
use aoc::cli;

use self::solutions::Solution;

fn main() -> Result<()> {
    let config = cli::Config::parse();
    match config.day {
        1 => solutions::day1::Day1::run(&config),
        2 => solutions::day2::Day2::run(&config),
        _ => Err(anyhow!("Day {} not implemented", config.day)),
    }
}
