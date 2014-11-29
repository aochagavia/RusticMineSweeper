/*

Implements the Mine Sweeper game based on the Board struct.

*/

#![feature(globs)]

use self::board::Board;
use self::board::ConsoleInput;

mod board;

fn main() {
    println!("----------------------");
	println!("|    Mine-Sweeper    |");
	println!("----------------------\n");

    println!("Welcome to Mine-Sweeper, please choose a level:");
    println!("1- Beginner (9x9 and 10 mines)");
    println!("2- Intermediate (16x16 and 40 mines)");

    let mut reader = std::io::stdin();
    let choice = reader.read_line().unwrap();
    let level = from_str(choice.trim_right()).unwrap_or(1);

    let mut board = Board::new(level);

    // Show instructions
    println!("--INSTRUCTIONS--\n");
	println!("Show the content of a square: s x y (example: s 2 3).");
	println!("Mark a square as a mine: m x y (example: m 2 3).\n");
    println!("Press enter to continue...");
    reader.read_line().ok().expect("IO Error");

    // Game loop
    while board.game_running() {
        // Clear screen
        println!("{}", "\n".repeat(50));

        // Show board and status
        print!("{}", board);
        println!("Mines left: {}", board.mines_left());

        // Get the next command
        print!("Command: ");
        board.console_input(reader.read_line().unwrap().trim_right());
    }

    // Ending
    print!("{}\n\n", board);
    if board.defeat() {
        println!("You lost! Better luck the next time!");
    } else {
        println!("Congratulations! You won!");
    }
}