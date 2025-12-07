use crate::day_4::grid::Grid;
use std::io::Lines;

mod grid;

pub fn day_4_solution(input: Lines<impl std::io::BufRead>) {
    let lines = input.map_while(Result::ok).collect::<Vec<_>>();
    let mut grid = Grid::from_lines(lines.iter().map(|x| x.as_str()).collect::<Vec<_>>());
    println!(
        "Number of reachable paper: {}",
        grid.reachable_paper_count()
    );
}
