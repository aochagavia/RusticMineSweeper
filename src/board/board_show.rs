/*

Implements the Show trait for Board. This way we are able to print the current Mine Sweeper game to the console.

*/

use std::fmt::{Show, Formatter, Result};
use super::Board;

impl Show for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Board
        // The numbers of the columns
        try!(write!(f, "     "));
        for x in range(0, self.width) {
            if x < 9 {
                try!(write!(f, "  "));
            }
            else {
                try!(write!(f, " "));
            }
            try!(write!(f, "{}", x + 1));
        }
        
        // A horizontal line
        try!(write!(f, "\n     {}\n", "_".repeat(3 * self.width)));
        
        // Vertical coordinate numbers + vertical line
        for y in range(0, self.height) {
            if y < 9 {
                try!(write!(f, "  "));
            }
            else {
                try!(write!(f, " "));
            }
            try!(write!(f, "{} |", y + 1));
            
            // Content of the board
            for x in range(0, self.width) {
                if self.get_square(x, y).is_hidden() {
                    if self.get_square(x, y).is_marked() { try!(write!(f, "  \\#")); }
                    else { try!(write!(f, "  *")); }
                }
                else if self.get_square(x, y).is_mine() { try!(write!(f, "  X")); }
                else if self.get_square(x, y).is_empty() { try!(write!(f, "   ")); }
                else { try!(write!(f, "  {}", self.get_square(x, y).get_value())); }
            }
            
            try!(write!(f, "\n\n"));
        }
        
        Ok(())
    }
}