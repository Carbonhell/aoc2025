mod problem;

use crate::day_6::problem::Problem;
use std::io::Lines;

pub fn day_6_solution(input: Lines<impl std::io::BufRead>) {
    let lines = input.map_while(Result::ok).collect::<Vec<_>>();
    let lines = lines.iter().map(|x| x.as_str()).collect::<Vec<_>>();
    let problems = Problem::from_table_l2r(&lines);
    let sum = problems.iter().map(|x| x.solve()).sum::<i64>();
    println!("Grand total of all the problems (normal math): {}", sum);
    let problems_r2l = Problem::from_table_r2l(&lines);
    let sum_r2l = problems_r2l.iter().map(|x| x.solve()).sum::<i64>();
    println!(
        "Grand total of all the problems (right-to-left): {}",
        sum_r2l
    );
}
