/*

Implements a trait which parses text input to actions.
There are two possible actions:
    -Show a square (s x y)
    -Mark a square as a mine (m x y)

*/

use super::Board;

pub trait ConsoleInput {
    fn console_input(&mut self, string: &str);
}

impl ConsoleInput for Board {
    fn console_input(&mut self, string: &str) {
        let mut input = string.split(' ');
        let action = input.next().unwrap_or("");

        let mut coords = input.map(|n| from_str::<uint>(n).unwrap_or(0));
        match (action, coords.next(), coords.next()) {
            ("s", Some(x), Some(y)) => self.show_square(x - 1, y - 1),
            ("m", Some(x), Some(y)) => self.mark_square(x - 1, y - 1),
            _ => ()
        }
    }
}