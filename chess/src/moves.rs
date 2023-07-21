use crate::pieces::Position;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    pub fn new(from: Position, to: Position) -> Self {
        Self { from, to }
    }

    pub fn inverse(&self) -> Self {
        Self {
            from: self.to,
            to: self.from,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];

        let from_file = files[self.from.x];
        let to_file = files[self.to.x];

        let from_rank = ranks[self.from.y];
        let to_rank = ranks[self.to.y];

        write!(f, "{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
    }
}