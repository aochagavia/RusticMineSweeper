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
            let spaces = if x < 9 { "  " } else { " " };
            try!(write!(f, "{}{}", spaces, x + 1));
        }

        // A horizontal line
        try!(write!(f, "\n     {}\n", "_".repeat(3 * self.width)));

        // Vertical coordinate numbers + vertical line
        for y in range(0, self.height) {
            let spaces = if y < 9 { "  " } else { " " };
            try!(write!(f, "{}{} |", spaces, y + 1));

            // The squares of the current line
            for x in range(0, self.width) {
                try!(write!(f, "{}", self.squares[x][y]));
            }

            try!(write!(f, "\n\n"));
        }

        Ok(())
    }
}