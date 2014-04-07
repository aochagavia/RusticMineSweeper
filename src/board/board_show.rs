/*

Implements the Show trait for Board. This way we are able to print the current Mine Sweeper game to the console.

*/

use std::fmt::{Show, Formatter, Result};
use board::Board;

impl Show for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut string = ~"";
        
        // Board
        // The numbers of the columns
        string.push_str("     ");
        for x in range(0, self.width) {
            if x < 9 {
                string.push_str("  ");
            }
            else {
                string.push_str(" ");
            }
            string.push_str((x + 1).to_str());
        }
        
        // A horizontal line
        string.push_str("\n");
        string.push_str(" ".repeat(5));
        string.push_str("_".repeat(3 * self.width));
        string.push_str("\n");
        
        // Vertical coordinate numbers + vertical line
        for y in range(0, self.height) {
            if y < 9 {
                string.push_str("  ");
            }
            else {
                string.push_str(" ");
            }
            string.push_str((y + 1).to_str() + " |");
            
            // Content of the board
            for x in range(0, self.width) {
                if self.squares[x][y].is_hidden() {
                    if self.squares[x][y].is_marked() { string.push_str("  #"); }
                    else { string.push_str("  *"); }
                }
                else if self.squares[x][y].is_mine() { string.push_str("  X"); }
                else if self.squares[x][y].is_empty() { string.push_str("   "); }
                else { string.push_str("  " + self.squares[x][y].get_value().to_str()); }
            }
            
            string.push_str("\n\n");
        }
        
        f.pad(string)
    }
}