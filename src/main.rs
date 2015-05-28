/*

Implements the Mine Sweeper game based on the Board struct.

*/

extern crate rand;
extern crate term_painter;

mod board;

use std::io::BufRead;

use self::board::Board;
use self::board::ConsoleInput;

fn main() {
    println!("----------------------");
	println!("|    Mine-Sweeper    |");
	println!("----------------------\n");

    println!("Welcome to Mine-Sweeper, please choose a level:");
    println!("1- Beginner (9x9 and 10 mines)");
    println!("2- Intermediate (16x16 and 40 mines)");

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let level = lines.next().unwrap().unwrap().trim_right().parse().unwrap_or(1);

    let mut board = Board::new(level);

    // Show instructions
    println!("--INSTRUCTIONS--\n");
	println!("Show the content of a square: s x y (example: s 2 3).");
	println!("Mark a square as a mine: m x y (example: m 2 3).\n");
    println!("Press enter to continue...");
    lines.next().unwrap().unwrap();

    // Game loop
    while board.game_running() {
        // Clear screen
        cls();

        // Show board and status
        print!("{}", board);
        println!("Mines left: {}", board.mines_left());

        // Get the next command
        print!("Command: ");
        board.console_input(lines.next().unwrap().unwrap().trim_right());
    }

    // Ending
    print!("{}\n\n", board);
    if board.defeat() {
        println!("You lost! Better luck the next time!");
    } else {
        println!("Congratulations! You won!");
    }
}

fn cls() {
    for _ in 0..50 {
        println!("");
    }
}
