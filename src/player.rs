use std::fmt;
use crate::cell::*;

pub struct Player<'a> {
    pub name: String,
    pub symbol: &'a Cell
}

impl<'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.symbol)
    }
}