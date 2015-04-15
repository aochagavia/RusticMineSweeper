/*

Implements an iterator object to go through all the squares of the board.

*/

use super::Board;
use super::square::Square;

pub struct SquareIter<'a> {
    board: &'a Board,
    n: u32
}

impl<'a> SquareIter<'a> {
    pub fn new(board: &'a Board) -> SquareIter<'a> {
        SquareIter { board: board, n: 0 }
    }

    fn n_to_2d(&self) -> (u32, u32) {
        (self.n / self.board.width(), self.n % self.board.width())
    }
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = &'a Square;

    fn next(&mut self) -> Option<&'a Square> {
        let (x, y) = self.n_to_2d();

        if !self.board.is_valid(x as i32, y as i32) {
            None
        } else {
            // Advance to the next square
            self.n += 1;

            Some(&self.board[(x, y)])
        }
    }
}

#[test]
fn square_iter_test() {
    let board = Board::new(1);
    assert_eq!(board.iter().count(), board.width() * board.height());
}