use crate::players::player::Player;
use crate::board::Board;

use std::io;
use std::io::Write;

pub struct Human;

impl Player for Human {
    fn best_move(&self, board: &Board) -> usize {
        let mut input = String::new();
        let move_: usize;

        board.print();

        print!("Enter move: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                move_ = input.trim().parse::<usize>().unwrap();
            }
            Err(error) => panic!("Error: {}", error),
        }

        move_
    }
}