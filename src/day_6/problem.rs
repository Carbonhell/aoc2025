use std::str::FromStr;

/// Describes both operators and operands (i32s)
#[derive(Debug, Eq, PartialEq)]
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
    pub fn from_table(table: Vec<&str>) -> Vec<Problem> {
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
        let problems = Problem::from_table(table);
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
        let problems = Problem::from_table(table);
        assert_eq!(
            problems.iter().map(|p| p.solve()).collect::<Vec<_>>(),
            vec![5, 8, 10, 32]
        );
    }
}
