use std::cmp::min;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}
#[derive(Debug, Eq, PartialEq)]
enum ManifoldComponentType {
    Beam,
    Splitter,
    EmptySpace,
}

impl TryFrom<char> for ManifoldComponentType {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Beam),
            '^' => Ok(Self::Splitter),
            '.' => Ok(Self::EmptySpace),
            _ => Err(()),
        }
    }
}

impl Display for ManifoldComponentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Beam => write!(f, "S"),
            Self::Splitter => write!(f, "^"),
            Self::EmptySpace => write!(f, "."),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ManifoldComponent {
    component_type: ManifoldComponentType,
    position: Position,
}

impl Display for ManifoldComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.component_type)
    }
}

#[derive(Debug)]
pub struct Manifold {
    components: Vec<ManifoldComponent>,
    columns: usize,
    rows: usize,
}

/// All methods of Manifold expect the components vec to be sorted by columns and rows.
impl Manifold {
    /// Builds a new instance of a Manifold.
    /// The inner components representation is guaranteed to be ordered by columns and rows.
    pub fn from_lines(lines: Vec<&str>) -> Self {
        Self {
            columns: lines.first().expect("There's no input to parse from").len(),
            rows: lines.len(),
            components: lines
                .into_iter()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(|(column, char)| {
                            let component_type = ManifoldComponentType::try_from(char).ok()?;
                            Some(ManifoldComponent {
                                component_type,
                                position: Position { x: column, y: row },
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        }
    }

    /// Counts the number of times the beam splits in the manifold.
    pub fn count_splits(&self) -> usize {
        let beam = self.get_beam();
        let mut splits = 0;
        let mut valid_columns = Vec::with_capacity(self.columns);
        valid_columns.push(beam.position.x);
        for row in 0..self.rows {
            valid_columns = valid_columns
                .into_iter()
                .map(|col| {
                    let cell = self
                        .components
                        .get((row * self.columns) + col)
                        .expect("The input is not sorted by columns and rows");
                    if cell.component_type == ManifoldComponentType::Splitter {
                        let previous = cell.position.x.saturating_sub(1);
                        let next = min(cell.position.x + 1, self.columns - 1);
                        splits += 1;
                        vec![previous, next]
                    } else {
                        vec![col]
                    }
                })
                .fold(Vec::new(), |mut acc, item| {
                    for el in item {
                        if !acc.contains(&el) {
                            acc.push(el);
                        }
                    }
                    acc
                });
        }
        splits
    }

    fn get_beam(&self) -> &ManifoldComponent {
        self.components
            .iter()
            .find(|el| el.component_type == ManifoldComponentType::Beam)
            .expect("The input contains no beam")
    }
}

impl Display for Manifold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.components.iter();
        for _ in 0..self.rows {
            for _ in 0..self.columns {
                write!(
                    f,
                    "{}",
                    iter.next().expect("Not enough components to display")
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
