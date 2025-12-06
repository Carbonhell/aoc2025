mod rotation;
mod dial;

use crate::day_1::dial::Dial;
use crate::day_1::rotation::Rotation;
use std::io::Lines;
use tracing::instrument;

#[instrument(skip(input))]
pub fn day_1_solution(input: Lines<impl std::io::BufRead>) {
    let rotations = input.map_while(Result::ok).map(|s| Rotation::from(s.as_str())).collect::<Vec<_>>();
    let mut dial = Dial::default();
    for rotation in rotations { dial.rotate(rotation); }
    println!("{}", dial.point_at_zero_counter);
}