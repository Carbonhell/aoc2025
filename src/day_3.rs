mod bank;

use std::io::Lines;
use tracing::instrument;
use crate::day_3::bank::Bank;

#[instrument(skip(input))]
pub fn day_3_solution(input: Lines<impl std::io::BufRead>) {
    let banks = input.map_while(Result::ok).map(|x| Bank::from(x.as_str())).collect::<Vec<_>>();
    println!("Total output joltage: {}", banks.iter().map(|x| x.joltage()).sum::<u32>())
}