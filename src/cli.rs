use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum, default_value_t = Command::Run)]
    pub command: Command,

    /// Which day to run
    #[arg(short, long)]
    pub day: usize,

    /// Which part to run
    #[arg(short, long, default_value_t = 1)]
    pub part: usize,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Command {
    Run,
    Fetch,
}

impl Args {
    pub fn input_path(&self) -> String {
        format!("inputs/day{}-{}.txt", self.day, self.part)
    }
}
