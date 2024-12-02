use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Which day to run
    #[arg(short, long)]
    pub day: usize,

    /// Which part to run
    #[arg(short, long, default_value_t = 1)]
    pub part: usize,
}

impl Config {
    pub fn input_path(&self) -> String {
        format!("inputs/day{}-{}.txt", self.day, self.part)
    }
}
