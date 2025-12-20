use clap::Parser;
use std::fs::File;
use std::io::BufRead;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

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
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let file = File::open(args.input_path).unwrap();
    let input_lines = std::io::BufReader::new(file).lines();
    match args.day {
        1 => day_1::day_1_solution(input_lines),
        2 => day_2::day_2_solution(input_lines),
        3 => day_3::day_3_solution(input_lines),
        4 => day_4::day_4_solution(input_lines),
        5 => day_5::day_5_solution(input_lines),
        6 => day_6::day_6_solution(input_lines),
        7 => day_7::day_7_solution(input_lines),
        _ => panic!("Day {} not implemented yet", args.day),
    }
}
