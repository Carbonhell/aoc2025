use crate::day_4::grid::Grid;
use std::io::Lines;

mod grid;

fn solve_first_half(lines: &Vec<&str>) {
    let mut grid = Grid::from_lines(lines);
    println!(
        "Number of reachable paper: {}",
        grid.reachable_paper_count()
    );
}

fn solve_second_half(lines: &Vec<&str>) {
    let mut grid = Grid::from_lines(lines);
    let mut removed_paper_rolls = 0;

    while let Some(count) = grid.remove_reachable_paper() {
        removed_paper_rolls += count;
    }
    println!(
        "Total number of rolls of paper that can be removed: {}",
        removed_paper_rolls
    )
}

pub fn day_4_solution(input: Lines<impl std::io::BufRead>) {
    let lines = input.map_while(Result::ok).collect::<Vec<_>>();
    let lines = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    solve_first_half(&lines);
    solve_second_half(&lines);
}
