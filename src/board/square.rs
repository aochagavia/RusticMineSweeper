/*

Implements the Square struct, which is the building block of the Board.

*/

use std::fmt::{self, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SquareContent {
    Value(u8), // Number of adjacent mines
    None,      // No adjacent mines
    Mine       // The square is a mine
}

#[derive(Clone)]
pub struct Square {
    content: SquareContent,
    hidden: bool,
    marked: bool
}

impl Square {
    pub fn new() -> Square {
        Square { content: SquareContent::None, hidden: true, marked: false }
    }

    // Setters
    pub fn set_mine(&mut self) {
        self.content = SquareContent::Mine;
    }

    pub fn set_value(&mut self, x: u8) {
        assert!(0 < x && x < 10);

        self.content = SquareContent::Value(x);
    }

    pub fn set_empty(&mut self) {
        self.content = SquareContent::None;
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn show(&mut self) {
        self.marked = false;
        self.hidden = false;
    }

    // Getters
    pub fn is_empty(&self) -> bool {
        self.content == SquareContent::None
    }

    pub fn is_mine(&self) -> bool {
        self.content == SquareContent::Mine
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hidden {
            if self.marked { return write!(f, "  #"); }
            else { return write!(f, "  *"); }
        }
        match self.content {
            SquareContent::Mine     => write!(f, "  X"),
            SquareContent::None     => write!(f, "   "),
            SquareContent::Value(x) => write!(f, "  {}", x)
        }
    }
}