/*

Implements an iterator object to go through all the squares of the board.

*/

use super::Board;
use super::square::Square;

pub struct SquareIter<'a> {
    board: &'a Board,
    x: uint,
    y: uint
}

impl<'a> SquareIter<'a> {
    pub fn new(board: &'a Board) -> SquareIter<'a> {
        SquareIter { board: board, x: 0, y: 0 }
    }
}

impl<'a> Iterator<&'a Square> for SquareIter<'a> {
    fn next(&mut self) -> Option<&'a Square> {
        if !self.board.is_valid(self.x as int, self.y as int) {
            return None;
        }
        
        let square = self.board.get_square(self.x, self.y);
        
        // Advance to the next square
        if self.x + 1 < self.board.width {
            self.x += 1;
        }
        else if self.x + 1 == self.board.width {
            self.x = 0;
            self.y += 1;
        }
        
        Some(square)
    }
}

#[test]
fn square_iter_test() {
    let board = Board::new(1);
    assert!(board.iter().count() == board.width * board.height);
}