mod bank;

use crate::day_3::bank::Bank;
use std::io::Lines;
use tracing::instrument;

#[instrument(skip(input))]
pub fn day_3_solution(input: Lines<impl std::io::BufRead>) {
    let mut banks = input
        .map_while(Result::ok)
        .map(|x| Bank::from(x.as_str()))
        .collect::<Vec<_>>();
    println!(
        "Total output joltage with 2 batteries: {}",
        banks.iter().map(|x| x.joltage()).sum::<u64>()
    );
    banks.iter_mut().for_each(|x| {
        x.set_max_enabled_batteries(12);
    });
    println!(
        "Total output joltage with 12 batteries: {}",
        banks.iter().map(|x| x.joltage()).sum::<u64>()
    );
}
