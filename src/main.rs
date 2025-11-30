use clap::Parser;

mod day_1;

/// Execute the solution for a specific day
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of the Advent of Code day to choose the solution to execute
    #[arg(short, long)]
    day: u8,
}
fn main() {
    let args = Args::parse();
    match args.day {
        1 => day_1::day_1_solution(),
        _ => panic!("Day {} not implemented yet", args.day),
    }
}
