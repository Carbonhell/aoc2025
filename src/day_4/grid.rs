use std::cmp::min;
use std::fmt::Display;
use tracing::debug;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Cell {
    Paper,
    ReachablePaper,
    Empty,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '@' => Cell::Paper,
            'x' => Cell::ReachablePaper,
            _ => panic!("Unexpected cell value: {}", c),
        }
    }
}

pub struct Grid {
    /// 1D representation of the grid, with each row represented consecutively, i.e. [[1,2,3],[4,5,6]] becomes [1,2,3,4,5,6]
    space: Vec<Cell>,
    columns: usize,
    rows: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.space.chunks(self.columns) {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|x| match x {
                        Cell::Paper => '@',
                        Cell::ReachablePaper => 'x',
                        Cell::Empty => '.',
                    })
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl Grid {
    /// Allows generating a grid from a vec of lines, where each line is a row with a char representing a cell.
    pub fn from_lines(lines: Vec<&str>) -> Self {
        if lines.is_empty() {
            panic!("Empty grid")
        }
        let columns = lines.first().unwrap().len();
        let rows = lines.len();
        let mut vec = Vec::with_capacity(lines.len() * columns);
        lines
            .iter()
            .for_each(|x| vec.extend(x.chars().map(Cell::from)));
        Self {
            space: vec,
            columns,
            rows,
        }
    }

    fn has_paper(&self, x: usize, y: usize) -> Option<bool> {
        self.space
            .get(x + y * self.columns)
            .map(|x| *x == Cell::Paper)
    }

    /// Verifies whether a cell can be reached by a forklift.
    /// A cell is considered reachable if there are fewer than four rolls of paper in the eight adjagent positions.
    fn is_reachable(&self, x: usize, y: usize) -> bool {
        let mut paper_found = 0;
        let previous_row = x.saturating_sub(1);
        let next_row = min(x + 1, self.rows - 1);
        let previous_column = y.saturating_sub(1);
        let next_column = min(y + 1, self.columns - 1);

        for row in previous_row..=next_row {
            for column in previous_column..=next_column {
                if row == x && column == y {
                    continue;
                }
                let cell = self.has_paper(row, column);
                if cell.unwrap_or(false) {
                    paper_found += 1;
                }
            }
        }
        paper_found < 4
    }

    pub fn mark_reachable_paper(&mut self) {
        let cells_to_mark = self
            .space
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == Cell::Paper)
            .filter_map(|(i, _)| {
                self.is_reachable(i % self.columns, i / self.rows)
                    .then_some(i)
            })
            .collect::<Vec<_>>();
        for i in cells_to_mark {
            self.space[i] = Cell::ReachablePaper;
        }
    }

    pub fn reachable_paper_count(&mut self) -> usize {
        self.mark_reachable_paper();
        debug!("space state\n{self}");

        self.space
            .iter()
            .filter(|x| **x == Cell::ReachablePaper)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_example() {
        let mut grid = Grid::from_lines(vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]);
        assert_eq!(grid.reachable_paper_count(), 13);
    }
}
