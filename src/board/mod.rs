/*

Implements the Board struct, which contains the Squares.
It contains different functions to control the gameplay.

*/

use rand::{ StdRng, Rng };
pub use board::square::Square;
pub use board::console_input::ConsoleInput;
use board::square_iter::SquareIter;

mod square;
mod console_input;
mod square_iter;
mod board_show;

pub struct Board {
    squares: ~[~[Square]],
    total_mines: uint,
    width: uint,
    height: uint
}

impl Board {
    // Initialize the board without assigning the mines yet
    pub fn new(level: uint) -> Board {
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
            
            _ => fail!("Level {} does not exist!", level)
        }
        
        // Use one column of "height" squares and copy it "width" times
        // When indexing overriding lands we will be able to use Vec instead of ~[T]
        // We could then use Vec::from_fn(|_| Square::new());
		let column = range(0u, height).map(|_| Square::new()).collect::<~[Square]>();
		let squares = range(0u, width).map(|_| column.clone()).collect::<~[~[Square]]>();
        
        Board { squares: squares, total_mines: total_mines, width: width, height: height }
    }
    
    // Set the mines randomly, but not in x, y
    pub fn add_mines(&mut self, x: uint, y: uint) {
        let mut rand = StdRng::new();
        
        let mut tempX; let mut tempY;
        let mut mines_set = 0;
        while mines_set < self.total_mines {
            tempX = rand.gen_range(0u, self.width);
            tempY = rand.gen_range(0u, self.height);
            
            if !(tempX == x && tempY == y) && !self.squares[tempX][tempY].is_mine() {
                self.squares[tempX][tempY].set_mine();
                mines_set += 1;
            }
        }
    }
    
    // Shows the square in x, y. Do nothing if the point is invalid
    pub fn show_square(&mut self, x: uint, y: uint) {
        if !self.is_valid(x as int, y as int) {
            return;
        }
        
        // We add the mines after the user has selected a square,
        // so he will not loose in his first turn
        if self.first_turn() {
            self.add_mines(x, y);
            self.generate_numbers();
        }
        
        // Empty squares get a special treatment to show "empty square islands"
        if self.squares[x][y].is_empty() {
            self.show_empty_squares(x as int, y as int);
        }
        // Normal numbers and mines are just shown
        else {
            self.squares[x][y].show();
        }
    }
    
    // Shows all empty squares connected to this one, the ones connected to them, and so on
    // The numbers in the bounds of the "empty squares island" will also be shown
    fn show_empty_squares(&mut self, x: int, y: int) {
        if !self.is_valid(x, y)
        || self.squares[x][y].is_mine()
        || !self.squares[x][y].is_hidden() {
            return;
        }
        else if !self.squares[x][y].is_empty() {
            self.squares[x][y].show();
            return;
        }
    
        self.squares[x][y].show();
        for aux_x in range(x - 1, x + 2) {
            for aux_y in range(y - 1, y + 2) {
                self.show_empty_squares(aux_x, aux_y);
            }
        }
    }
    
    // Marks a square to indicate that you expect it to be a mine
    pub fn mark_square(&mut self, x: uint, y: uint) {
        if self.is_valid(x as int, y as int) {
            self.squares[x][y].mark();
        }
    }
    
    // Assigns to each square the number of surrounding mines
    fn generate_numbers(&mut self) {  
        for x in range(0, self.width) {
            for y in range(0, self.height) {
                if !self.squares[x][y].is_mine() {
                    match self.surrounding_mines(x, y) {
                        0 => self.squares[x][y].set_empty(),
                        n => self.squares[x][y].set_value(n)
                    }
                }
            }
        }
    }
    
    // Return true if it is the first move of the player
    // This happens when all squares are hidden
    pub fn first_turn(&self) -> bool {
        self.iter().count(|square| !square.is_hidden()) == 0
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
        self.iter().count(|square| square.is_hidden()) == self.total_mines
    }
    
    // Returns the amount of mines that have not been marked
    pub fn mines_left(&self) -> int {
        self.total_mines as int - self.marked_mines() as int
    }
    
    // Returns the amount of mines surrounding the point x, y
    fn surrounding_mines(&self, x: uint, y: uint) -> uint {
        let mut count = 0;
        for aux_x in range(x as int - 1, x as int + 2) {
            for aux_y in range(y as int - 1, y as int + 2) {
                if self.is_valid(aux_x, aux_y) && self.squares[aux_x as uint][aux_y as uint].is_mine() {
                    count += 1;
                }
            }
        }
        
        count
    }
    
    // Returns the amount of squares marked as mines
    fn marked_mines(&self) -> uint {
        self.iter().count(|square| square.is_marked())
    }
    
    // Returns true if the coordinates are within the bounds of the board
    fn is_valid(&self, x: int, y: int) -> bool {
        0 <= x && x < self.width as int && 0 <= y && y < self.height as int
    }
    
    fn iter<'a>(&'a self) -> SquareIter<'a> {
        SquareIter::new(self)
    }
}