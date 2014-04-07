/*

Implements a trait which parses text input to actions.
There are two possible actions:
    -Show a square (s x y)
    -Mark a square as a mine (m x y)

*/

use board::Board;

pub trait ConsoleInput {
    fn console_input(&mut self, string: &str);
}

impl ConsoleInput for Board {
    fn console_input(&mut self, string: &str) {
        let mut input = string.split(' ');
        let action = input.next().unwrap_or("");
        
        match (from_str::<uint>(input.next().unwrap_or("")), from_str::<uint>(input.next().unwrap_or(""))) {
            (Some(x), Some(y)) if x > 0 && y > 0 => {
                if action == "s" { self.show_square(x - 1, y - 1); }
                else if action == "m" { self.mark_square(x - 1, y - 1); }
            }
            
            _ => ()
        }
    }
}