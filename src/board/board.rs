/*

Implements the Board struct, which contains the Squares.
It contains different functions to control the gameplay.

*/

use std::fmt::{Display, Formatter, Result};
use std::iter;
use std::ops::{Index, IndexMut};

use rand::{thread_rng, Rng};
use term_painter::{ToStyle, Color};

use super::square::Square;
use super::square_iter::SquareIter;


pub struct Board {
    squares: Vec<Vec<Square>>,
    total_mines: u32,
    width: u32,
    height: u32
}

impl Index<(u32, u32)> for Board {
    type Output = Square;

    fn index(&self, (x, y): (u32, u32)) -> &Square {
        &self.squares[x as usize][y as usize]
    }
}

impl IndexMut<(u32, u32)> for Board {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Square {
        &mut self.squares[x as usize][y as usize]
    }
}

impl Board {
    // Initialize the board without assigning the mines yet
    pub fn new(level: u8) -> Board {
        let total_mines;
        let width; let height;
        match level {
            1 => {
                total_mines = 10;
                width = 9;
                height = 9;
            }
            2 => {
                total_mines = 40;
                width = 16;
                height = 16;
            }
            _ => panic!("Level {} does not exist!", level)
        }

        // Use one column of "height" squares and copy it "width" times
		let column: Vec<_> = (0..height).map(|_| Square::new()).collect();
		let squares = (0..width).map(|_| column.clone()).collect();

        Board { squares: squares, total_mines: total_mines, width: width, height: height }
    }

    // Set the mines randomly, but not in x, y
    pub fn add_mines(&mut self, x: u32, y: u32) {
        let mut rand = thread_rng();

        let mut temp_x; let mut temp_y;
        let mut mines_set = 0;
        while mines_set < self.total_mines {
            temp_x = rand.gen_range(0, self.width);
            temp_y = rand.gen_range(0, self.height);

            if !(temp_x == x && temp_y == y) && !self[(temp_x, temp_y)].is_mine() {
                self[(temp_x, temp_y)].set_mine();
                mines_set += 1;
            }
        }
    }

    // Shows the square in x, y. Do nothing if the point is invalid
    pub fn show_square(&mut self, x: u32, y: u32) {
        if !self.is_valid(x as i32, y as i32) {
            return;
        }

        // We add the mines after the user has selected a square,
        // so he will not lose the first time he shows a square
        if self.first_turn() {
            self.add_mines(x, y);
            self.generate_numbers();
        }

        // Empty squares get a special treatment to show "empty square islands"
        if self[(x, y)].is_empty() {
            self.show_empty_squares(x as i32, y as i32);
        } else {
            self[(x, y)].show();
        }
    }

    // Shows all empty squares connected to this one, the ones connected to them, and so on
    // The numbers in the bounds of the "empty squares island" will also be shown
    fn show_empty_squares(&mut self, x: i32, y: i32) {
        let (ux, uy) = (x as u32, y as u32);

        if !self.is_valid(x, y)
        || self[(ux, uy)].is_mine()
        || !self[(ux, uy)].is_hidden() {
            return;
        }
        else if !self[(ux, uy)].is_empty() {
            self[(ux, uy)].show();
            return;
        }

        self[(ux, uy)].show();
        for aux_x in (x - 1..x + 2) {
            for aux_y in (y - 1..y + 2) {
                self.show_empty_squares(aux_x, aux_y);
            }
        }
    }

    // Marks a square to indicate that you expect it to be a mine
    pub fn mark_square(&mut self, x: u32, y: u32) {
        if self.is_valid(x as i32, y as i32) {
            self[(x, y)].mark();
        }
    }

    // Assigns to each square the number of surrounding mines
    fn generate_numbers(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if !self[(x, y)].is_mine() {
                    match self.surrounding_mines(x, y) {
                        0 => self[(x, y)].set_empty(),
                        n => self[(x, y)].set_value(n)
                    }
                }
            }
        }
    }

    // Return true if it is the first "show" of the player
    // This happens when all squares are hidden
    pub fn first_turn(&self) -> bool {
        self.iter().all(|square| square.is_hidden())
    }

    // Returns true if the game is running
    pub fn game_running(&self) -> bool {
        !(self.defeat() || self.no_moves_left())
    }

    // Returns true if the player has lost
    // This happens when a mine is shown
    pub fn defeat(&self) -> bool {
        self.iter().any(|square| !square.is_hidden() && square.is_mine())
    }

    // Returns true if there are no possible moves left
    // This happens when all squares are shown except the mines
    pub fn no_moves_left(&self) -> bool {
        self.iter().filter(|square| square.is_hidden()).count() as u32 == self.total_mines
    }

    // Returns the amount of mines that have not been marked
    pub fn mines_left(&self) -> i32 {
        self.total_mines as i32 - self.marked_mines() as i32
    }

    // Returns the amount of mines surrounding the point x, y
    fn surrounding_mines(&self, x: u32, y: u32) -> u8 {
        (x as i32 - 1 .. x as i32 + 2)
            .zip(y as i32 - 1 .. y as i32 + 2)
            .filter(|&(aux_x, aux_y)| self.is_valid(aux_x, aux_y) &&
                                      self[(aux_x as u32, aux_y as u32)].is_mine())
            .count() as u8
    }

    // Returns the amount of squares marked as mines
    fn marked_mines(&self) -> u32 {
        self.iter().filter(|square| square.is_marked()).count() as u32
    }

    // Returns true if the coordinates are within the bounds of the board
    pub fn is_valid(&self, x: i32, y: i32) -> bool {
        0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32
    }

    pub fn iter<'a>(&'a self) -> SquareIter<'a> {
        SquareIter::new(self)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

// Implement display to print the board in a user-friendly way
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // The numbers of the columns
        try!(write!(f, "     "));
        for x in 0..self.width {
            let spaces = if x < 9 { "  " } else { " " };
            try!(write!(f, "{}{}", spaces, Color::Green.paint((x + 1).to_string())));
        }

        // A horizontal line
        try!(write!(f, "\n     {}\n", iter::repeat("_").take(3 * self.width as usize).collect::<String>()));

        // Vertical coordinate numbers + vertical line
        for y in 0..self.height {
            let spaces = if y < 9 { "  " } else { " " };
            try!(write!(f, "{}{} |", spaces, Color::Green.paint((y + 1).to_string())));

            // The squares of the current line
            for x in 0..self.width {
                try!(write!(f, "{}", self[(x, y)]));
            }

            try!(write!(f, "\n\n"));
        }

        Ok(())
    }
}
