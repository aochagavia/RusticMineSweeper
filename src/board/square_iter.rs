/*

Implements an iterator object to go through all the squares of the board.

*/

use super::Board;
use super::square::Square;

pub struct SquareIter<'a> {
    board: &'a Board,
    n: uint
}

impl<'a> SquareIter<'a> {
    pub fn new(board: &'a Board) -> SquareIter<'a> {
        SquareIter { board: board, n: 0 }
    }

    fn n_to_2d(&self) -> (uint, uint) {
        (self.n / self.board.width, self.n % self.board.width)
    }
}

impl<'a> Iterator<&'a Square> for SquareIter<'a> {
    fn next(&mut self) -> Option<&'a Square> {
        let (x, y) = self.n_to_2d();

        if !self.board.is_valid(x as int, y as int) {
            None
        } else {
            // Advance to the next square
            self.n += 1;

            Some(&self.board.squares[x][y])
        }
    }
}

#[test]
fn square_iter_test() {
    let board = Board::new(1);
    assert!(board.iter().count() == board.width * board.height);
}