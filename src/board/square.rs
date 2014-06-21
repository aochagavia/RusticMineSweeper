/*

Implements the Square struct, which is the building block of the Board.

*/

use std::fmt;

#[deriving(Clone)]
enum SquareContent {
    Value(uint), // Number of adjacent mines
    None,        // No adjacent mines
    Mine         // The square is a mine
}

#[deriving(Clone)]
pub struct Square {
    content: SquareContent,
    hidden: bool,
    marked: bool
}

impl Square {
    pub fn new() -> Square {
        Square { content: None, hidden: true, marked: false }
    }

    // Setters
    pub fn set_mine(&mut self) {
        self.content = Mine;
    }

    pub fn set_value(&mut self, x: uint) {
        self.content = Value(x);
    }

    pub fn set_empty(&mut self) {
        self.content = None;
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
        match self.content {
            None => true,
            _ => false
        }
    }

    pub fn is_mine(&self) -> bool {
        match self.content {
            Mine => true,
            _ => false
        }
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
}

impl fmt::Show for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hidden {
            if self.marked { return write!(f, "  #"); }
            else { return write!(f, "  *"); }
        }
        match self.content {
            Mine     => write!(f, "  X"),
            None     => write!(f, "   "),
            Value(x) => write!(f, "  {}", x)
        }
    }
}