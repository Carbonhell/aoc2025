use std::str::FromStr;
use tracing::debug;

/// Describes both operators and operands (i32s)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Op {
    Mul,
    Add,
    Expr(i64),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Op::Mul),
            "+" => Ok(Op::Add),
            _ => Ok(s
                .parse::<i64>()
                .map(Op::Expr)
                .expect("Unexpected string found while parsing operator/operand")),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    ops: Vec<Op>,
}

impl Problem {
    /// Builds a vec of problems from a matrix
    /// It is assumed that each column has an equal amount of numbers, with the last element being an
    /// operation, and that each column is separated by an arbitrary amount of spaces.
    /// The result will be a mathematical expression with a series of operands followed by a single operator to apply to them.
    pub fn from_table_l2r(table: &Vec<&str>) -> Vec<Problem> {
        let x = table
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .map(|y| y.parse::<Op>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // Transpose the vec of vecs to get columns instead of rows - shamelessly taken from https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
        // It is safe to assume each row will have the same amount of columns, as the amount of columns is equal to the amount of problems we have
        let columns = x[0].len();
        let mut iters: Vec<_> = x.into_iter().map(|x| x.into_iter()).collect();
        let problems: Vec<_> = (0..columns)
            .map(|_| Problem {
                ops: iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<_>>(),
            })
            .collect();

        problems
    }

    /// Builds a vec of problems from a matrix, reading from right to left and considering number positions in each column.
    ///
    /// SAFETY:
    /// It is assumed that each column may only contain spaces at its start or at the end, but not between digits.
    /// If a space is present between digits, any digit above the space *will* be ignored, as it is unclear how such case should be handled from the instructions.
    pub fn from_table_r2l(table: &Vec<&str>) -> Vec<Problem> {
        let columns = table[0].len();
        let mut iter_of_chars = table.iter().map(|x| x.chars().rev()).collect::<Vec<_>>();

        // Transpose the vec of vecs to get columns instead of rows - shamelessly taken from https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
        // It is safe to assume each row will have the same amount of columns, as the amount of columns is equal to the amount of problems we have
        let mut skip_delimiter_column = false;
        let mut problems = vec![];
        let orphan_operands: Vec<_> = (0..columns).fold(vec![], |mut acc, col_idx| {
            debug!(%col_idx, "Processing column");
            let mut column = iter_of_chars
                .iter_mut()
                .map(|n| {
                    n.next().expect(
                        "Couldn't get next column - ensure all rows have the same amount of chars",
                    )
                })
                // At this point, we'll have an iterator of chars of the column being processed, and we need to convert it back to an operand
                // to_string can be optimized - we just need it because we're dealing with chars
                .rev(); // Start from the bottom cell, either an operator or a space, followed by the operand digits
            if skip_delimiter_column {
                // Process the column by just skipping it
                column.for_each(|_| {});
                skip_delimiter_column = false;
                return acc;
            }
            // Store the operand temporarily, to use it later on to decide whether to fold the accumulator or continue to the next column
            let operator = column
                .next()
                .expect("Unexpected end of column while parsing operator/operand");
            let operand = column
                .filter(|x| x != &' ')
                .map(|x| {
                    x.to_digit(10)
                        .expect("Unexpected non-digit character found while parsing operand")
                        as i64
                })
                .enumerate()
                .fold(0, |acc, (i, digit)| acc + (digit * 10_i64.pow(i as u32)));
            debug!(?operator, ?operand, "Pushing operand");
            acc.push(Op::Expr(operand));
            if operator != ' ' {
                let op = operator
                    .to_string()
                    .parse::<Op>()
                    .expect("Failed to parse operator");
                debug!(?op, ?acc, "Found operator, flushing problem");
                // Flush the accumulator
                acc.push(op);
                problems.push(Problem { ops: acc.clone() });
                acc.clear();
                // Signal to the next iteration that we need to skip the delimiter column
                skip_delimiter_column = true;
            }
            acc
        });
        if !orphan_operands.is_empty() {
            panic!(
                "The table is not properly formatted. There are orphan operands that couldn't be folded into a valid problem."
            );
        }

        problems
    }

    /// Solves the problem by evaluating the operator at the end of the problem to all the operands.
    /// Panics if the accumulator does not hold a single value at the end, which means the problem was incorrectly formulated.
    pub fn solve(&self) -> i64 {
        let accumulator =
            self.ops
                .iter()
                .fold(Vec::with_capacity(self.ops.len()), |mut acc, el| match el {
                    Op::Expr(x) => {
                        acc.push(*x);
                        acc
                    }
                    Op::Add => {
                        vec![acc.iter().sum::<i64>()]
                    }
                    Op::Mul => {
                        vec![acc.into_iter().product::<i64>()]
                    }
                });
        if accumulator.len() != 1 {
            panic!(
                "Couldn't reduce the problem to a single value. The expression was incorrectly formulated."
            );
        }
        accumulator[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_simple_op_parsing() {
        let ops = vec!["1", "2", "+", "*"]
            .iter()
            .map(|el| el.parse::<Op>().expect("Failed to parse valid Op"))
            .collect::<Vec<_>>();
        assert_eq!(ops, vec![Op::Expr(1), Op::Expr(2), Op::Add, Op::Mul]);
    }

    #[test]
    fn test_problems_table_parsing() {
        let table = vec!["1 2 3 4", "5 6 7 8", "* + + *"];
        let problems = Problem::from_table_l2r(&table);
        assert_eq!(
            problems,
            vec![
                Problem {
                    ops: vec![Op::Expr(1), Op::Expr(5), Op::Mul]
                },
                Problem {
                    ops: vec![Op::Expr(2), Op::Expr(6), Op::Add]
                },
                Problem {
                    ops: vec![Op::Expr(3), Op::Expr(7), Op::Add,]
                },
                Problem {
                    ops: vec![Op::Expr(4), Op::Expr(8), Op::Mul]
                }
            ]
        )
    }

    #[test]
    fn test_problems_solving() {
        let table = vec!["1 2 3 4", "5 6 7 8", "* + + *"];
        let problems = Problem::from_table_l2r(&table);
        assert_eq!(
            problems.iter().map(|p| p.solve()).collect::<Vec<_>>(),
            vec![5, 8, 10, 32]
        );
    }

    #[test]
    fn test_problems_table_parsing_r2l_example() {
        let table = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];
        let problems = Problem::from_table_r2l(&table);
        assert_eq!(
            problems,
            vec![
                Problem {
                    ops: vec![Op::Expr(4), Op::Expr(431), Op::Expr(623), Op::Add]
                },
                Problem {
                    ops: vec![Op::Expr(175), Op::Expr(581), Op::Expr(32), Op::Mul]
                },
                Problem {
                    ops: vec![Op::Expr(8), Op::Expr(248), Op::Expr(369), Op::Add]
                },
                Problem {
                    ops: vec![Op::Expr(356), Op::Expr(24), Op::Expr(1), Op::Mul]
                }
            ]
        )
    }
}
