/*

Implements the Board struct, which contains the Squares.
It contains different functions to control the gameplay.

*/

mod board;
pub mod console_input;
mod square;
mod square_iter;

pub use self::board::Board;
pub use self::console_input::ConsoleInput;