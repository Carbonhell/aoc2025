use std::fs::File;
use std::io::BufRead;
use clap::Parser;

mod day_1;

/// Execute the solution for a specific day
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the Advent of Code day to choose the solution to execute
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    input_path: String,
}
fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    let args = Args::parse();
    let file = File::open(args.input_path).unwrap();
    let input_lines = std::io::BufReader::new(file).lines();
    match args.day {
        1 => day_1::day_1_solution(input_lines),
        _ => panic!("Day {} not implemented yet", args.day),
    }
}
