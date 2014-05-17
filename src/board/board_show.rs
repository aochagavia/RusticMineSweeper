/*

Implements the Show trait for Board. This way we are able to print the current Mine Sweeper game to the console.

*/

use std::fmt::{Show, Formatter, Result};
use super::Board;

impl Show for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut buf = StrBuf::new();
        
        // Board
        // The numbers of the columns
        buf.push_str("     ");
        for x in range(0, self.width) {
            if x < 9 {
                buf.push_str("  ");
            }
            else {
                buf.push_str(" ");
            }
            buf.push_str((x + 1).to_str());
        }
        
        // A horizontal line
        buf.push_str("\n");
        buf.push_str(" ".repeat(5));
        buf.push_str("_".repeat(3 * self.width));
        buf.push_str("\n");
        
        // Vertical coordinate numbers + vertical line
        for y in range(0, self.height) {
            if y < 9 {
                buf.push_str("  ");
            }
            else {
                buf.push_str(" ");
            }
            buf.push_str((y + 1).to_str() + " |");
            
            // Content of the board
            for x in range(0, self.width) {
                if self.get_square(x, y).is_hidden() {
                    if self.get_square(x, y).is_marked() { buf.push_str("  #"); }
                    else { buf.push_str("  *"); }
                }
                else if self.get_square(x, y).is_mine() { buf.push_str("  X"); }
                else if self.get_square(x, y).is_empty() { buf.push_str("   "); }
                else { buf.push_str("  " + self.get_square(x, y).get_value().to_str()); }
            }
            
            buf.push_str("\n\n");
        }
        
        f.pad(buf.as_slice())
    }
}