mod ranges;

use crate::day_5::ranges::Ranges;
use std::io::Lines;

pub fn day_5_solution(input: Lines<impl std::io::BufRead>) {
    let mut ranges = vec![];
    let mut ingredient_ids = vec![];
    let mut parsing_ranges = true;
    input.map_while(Result::ok).for_each(|line| {
        if line.is_empty() {
            parsing_ranges = false;
            return;
        }
        if parsing_ranges {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            ranges.push((start, end));
        } else {
            ingredient_ids.push(line.parse::<usize>().unwrap());
        }
    });
    let ranges = Ranges::new(ranges);
    let fresh_ingredients = ingredient_ids
        .iter()
        .filter(|x| ranges.contains(**x))
        .count();
    println!("Fresh ingredients available: {}", fresh_ingredients);
    println!("Total amount of fresh ingredient IDs: {}", ranges.count());
}
