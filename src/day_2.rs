mod id;

use crate::day_2::id::{generate_invalid_ids_at_least_twice, generate_invalid_ids_exactly_twice};
use std::io::Lines;
use tracing::instrument;

#[instrument(skip(input))]
pub fn day_2_solution(mut input: Lines<impl std::io::BufRead>) {
    // The input will always be made of a single line representing a comma separated list of ranges
    let input = input.next().unwrap().unwrap();
    let input = input
        .split(',')
        .map(|s| {
            let mut parts = s.split('-').map(|part| part.parse::<u64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect::<Vec<_>>();
    let invalid_ids_exactly_2 = input
        .iter()
        .flat_map(|x| generate_invalid_ids_exactly_twice(*x))
        .collect::<Vec<_>>();

    println!(
        "Sum of invalid IDs (with exactly 2 repeated subsequences): {:?}",
        invalid_ids_exactly_2.iter().sum::<u64>()
    );
    let invalid_ids_at_least_2 = input
        .iter()
        .flat_map(|x| generate_invalid_ids_at_least_twice(*x))
        .collect::<Vec<_>>();
    println!(
        "Sum of invalid IDs (with at least 2 repeated subsequences): {:?}",
        invalid_ids_at_least_2.iter().sum::<u64>()
    );
}
