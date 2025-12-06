use std::fmt::{Display, Formatter};
use tracing::debug;

#[derive(Debug)]
pub enum RotationDirection {
    Left,
    Right,
}

impl Display for RotationDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RotationDirection::Left => write!(f, "L"),
            RotationDirection::Right => write!(f, "R"),
        }
    }
}


#[derive(Debug)]
pub struct Rotation {
    pub(crate) direction: RotationDirection,
    pub(crate) steps: u32,
}

impl From<&str> for Rotation {
    fn from(s: &str) -> Self {
        debug!(%s, "Parsing rotation");
        let first_char = s.chars().next().unwrap();
        Self {
            direction: match first_char {
                'L' => RotationDirection::Left,
                'R' => RotationDirection::Right,
                _ => panic!("Invalid rotation direction"),
            },
            steps: s[1..].parse().unwrap(),
        }
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.direction, self.steps)
    }
}
