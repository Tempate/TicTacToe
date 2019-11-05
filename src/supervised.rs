use nn::{NN, HaltCondition};

include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

use crate::board;
use crate::alphabeta;

lazy_static! {
    static ref ALL_BOARDS: Vec<board::Board> = leaf_boards(&board::Board{ tiles: [0, 0], turn: 0 });
}

pub struct SupervisedNetwork {
    nn: NN,
    epochs: u32,
    learning_rate: f64,
}

impl SupervisedNetwork {
    pub fn init() -> SupervisedNetwork {
        let size = [27, 81, 81, 27, 9];
        
        return SupervisedNetwork{ 
            nn: NN::new(&size),
            epochs: 1,
            learning_rate: 0.03
        };
    }

    pub fn train(&mut self) {
        let mut data = Vec::new();

        for board in ALL_BOARDS.iter() {
            let (score, m) = alphabeta::alphabeta(&board, -2, 2);
            
            // Creates an array of all possible moves and assigns
            // a one to the chosen move.
            let mut target: [f64; N2] = [0.0; N2];
            target[m-1] = 1.0;

            // Parses the board for it to be processed.
            let input_board = board.to_binary();

            // Adds the tuple of input and target values to the training data.
            data.push((input_board.to_vec(), target.to_vec()));
        }

        self.nn.train(&data)
            .halt_condition( HaltCondition::Epochs(self.epochs) )
            .rate( self.learning_rate )
            .go();        
    }

    // Feeds a position into the NN and returns the chosen move.
    pub fn play(&self, board: &board::Board) -> usize {
        let output = self.nn.run(&board.to_binary().to_vec());

        // Gets the index of the highest value in output

        let mut best_move: usize = 0;
        let mut highest_score: f64 = -2.0;

        for i in 0..output.len() {
            if output[i] > highest_score {
                highest_score = output[i];
                best_move = i + 1;
            }
        }

        assert!(best_move >= 1 && best_move <= N2);

        return best_move;
    }

    // Runs the network on all possible positions to see how many
    // times it gets the correct move.
    pub fn test(&self) {
        let mut correct = 0.0;

        for board in ALL_BOARDS.iter() {
            let guess = self.play(&board);
            let (_, move_) = alphabeta::alphabeta(&board, -2, 2);

            if move_ == guess {
                correct += 1.0;
            }
        }

        println!("Correct move: {:.5} %", correct * 100.0 / ALL_BOARDS.len() as f64);
    }
}

// Generates all the possible unfinished boards
fn leaf_boards(board: &board::Board) -> Vec<board::Board> {
    assert!(board.state() == board::State::Unfinished);

    let moves: Vec<usize> = board.gen_moves();
    assert!(moves.len() > 0);

    let mut boards: Vec<board::Board> = vec![*board, board.inverse()];

    for m in moves {
        let mut new_board = board.clone();
        new_board.make(m);

        if new_board.state() == board::State::Unfinished {
            boards.append(&mut leaf_boards(&new_board));
        }
    }

    return boards;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_all_boards() {
        let blank_board = board::Board { tiles: [0, 0], turn: 0 };
        let boards = leaf_boards(&blank_board);

        let test_boards = [
            board::Board { tiles: [0, 0], turn: 0 },
            board::Board { tiles: [0, 0], turn: 1 },
            board::Board { tiles: [0b100100010, 0b010010100], turn: 0 },
            board::Board { tiles: [0b100100010, 0b010010100], turn: 1 },
            board::Board { tiles: [0b010010100, 0b100100010], turn: 0 },
            board::Board { tiles: [0b010010100, 0b100100010], turn: 1 },
            board::Board { tiles: [0b000010000, 0b000000010], turn: 0 },
            board::Board { tiles: [0b000010000, 0b000000010], turn: 1 },
            board::Board { tiles: [0b000000010, 0b000010000], turn: 0 },
            board::Board { tiles: [0b000000010, 0b000010000], turn: 1 }
        ];

        for board in test_boards.iter() {
            boards.contains(board);
        }
    }
}