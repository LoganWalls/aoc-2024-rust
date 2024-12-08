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
        3 => solutions::day3::Day3::run(&config),
        4 => solutions::day4::Day4::run(&config),
        5 => solutions::day5::Day5::run(&config),
        6 => solutions::day6::Day6::run(&config),
        7 => solutions::day7::Day7::run(&config),
        _ => Err(anyhow!("Day {} not implemented", config.day)),
    }
}
