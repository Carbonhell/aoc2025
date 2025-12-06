mod id;

use crate::day_2::id::generate_invalid_ids;
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
    let invalid_ids = input
        .iter()
        .map(|x| generate_invalid_ids(*x))
        .flatten()
        .collect::<Vec<_>>();
    println!("Sum of invalid IDs: {:?}", invalid_ids.iter().sum::<u64>());
}
