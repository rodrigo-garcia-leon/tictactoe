use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Empty => write!(f, " "),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}
