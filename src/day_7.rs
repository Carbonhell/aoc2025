mod manifold;

use crate::day_7::manifold::Manifold;
use std::io::Lines;

pub fn day_7_solution(input: Lines<impl std::io::BufRead>) {
    let lines = input.map_while(Result::ok).collect::<Vec<_>>();
    let lines = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    let manifold = Manifold::from_lines(lines);
    println!("{}", manifold);
    println!(
        "Number of times the beam splits: {}",
        manifold.count_splits()
    );
}
