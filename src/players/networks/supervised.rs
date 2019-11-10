use nn::{NN, HaltCondition};

use crate::constants::*;

use crate::board;
use crate::board::{Board, State};

use crate::players::{
    player::Player, 
    networks::network::Network,
    alphabeta::AlphaBeta    
};

lazy_static! {
    static ref ALL_BOARDS: Vec<Board> = {
        let mut v = Vec::new();
        leaf_boards(Board{tiles: [0, 0], turn: 0}, &mut v);
        v
    };
}

pub struct SupervisedNetwork {
    pub nn: NN,
    pub epochs: u32,
    pub learning_rate: f64,
}

impl SupervisedNetwork {
    pub fn init() -> SupervisedNetwork {
        let size = [27, 81, 27, 9];
        
        SupervisedNetwork{ 
            nn: NN::new(&size),
            epochs: 1,
            learning_rate: 0.3
        }
    }
}

impl Network for SupervisedNetwork {
    // Feeds a position into the NN and returns the chosen move.
    fn play(&self, board: &Board) -> usize {
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

        best_move
    }

    fn train(&mut self) {
        let mut lr = self.learning_rate;

        let mut data = Vec::new();

        let ab = AlphaBeta{};

        for board in ALL_BOARDS.iter() {
            let move_ = ab.best_move(&board);
            
            // Creates an array of all possible moves and assigns
            // a one to the chosen move.
            let mut target: [f64; N2] = [0.0; N2];
            target[move_ - 1] = 1.0;

            // Parses the board for it to be processed.
            let input_board = board.to_binary();

            // Adds the tuple of input and target values to the training data.
            data.push((input_board.to_vec(), target.to_vec()));
        }

        for _ in 0..100 {
            self.nn.train(&data)
                .halt_condition( HaltCondition::Epochs(self.epochs) )
                .rate(lr)
                .go();

            lr *= 0.9;
        }
    }

    // Runs the network on all possible positions to see how many
    // times it gets the correct move.
    fn test(&self) {
        let mut correct = 0.0;
        let ab = AlphaBeta{};

        for board in ALL_BOARDS.iter() {
            let guess = self.play(&board);
            let move_ = ab.best_move(&board);

            if move_ == guess {
                correct += 1.0;
            }
        }

        println!("Correct move: {:.5} %", correct * 100.0 / ALL_BOARDS.len() as f64);
    }
}

// Generates all the possible unfinished boards
fn leaf_boards(board: Board, boards: &mut Vec<Board>) {
    assert!(board.state() == State::Unfinished);

    let moves: Vec<usize> = board.gen_moves();
    assert!(moves.len() > 0);

    for b in boards.iter() {
        if  board.tiles[board::PLAYER1] == b.tiles[board::PLAYER1] &&
            board.tiles[board::PLAYER2] == b.tiles[board::PLAYER2] &&
            board.turn == b.turn {
                return;
        }
    }

    boards.push(board);
    boards.push(board.inverse());

    for m in moves {
        let mut new_board = board.clone();
        new_board.make(m);

        if new_board.state() == State::Unfinished {
            leaf_boards(new_board, boards);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_all_boards() {
        let mut boards: Vec<Board> = Vec::new();
        
        leaf_boards(Board{tiles: [0, 0], turn: 0}, &mut boards);

        let test_boards = [
            Board { tiles: [0, 0], turn: 0 },
            Board { tiles: [0, 0], turn: 1 },
            Board { tiles: [0b100100010, 0b010010100], turn: 0 },
            Board { tiles: [0b100100010, 0b010010100], turn: 1 },
            Board { tiles: [0b010010100, 0b100100010], turn: 0 },
            Board { tiles: [0b010010100, 0b100100010], turn: 1 },
            Board { tiles: [0b000010000, 0b000000010], turn: 0 },
            Board { tiles: [0b000010000, 0b000000010], turn: 1 },
            Board { tiles: [0b000000010, 0b000010000], turn: 0 },
            Board { tiles: [0b000000010, 0b000010000], turn: 1 }
        ];

        for board in test_boards.iter() {
            boards.contains(board);
        }
    }

    #[test]
    fn test_repeated_boards() {
        for i in 0..ALL_BOARDS.len() {
            let board1 = ALL_BOARDS[i];

            for j in i+1..ALL_BOARDS.len() {
                let board2 = ALL_BOARDS[j];

                assert!(
                    board1.tiles[board::PLAYER1] != board2.tiles[board::PLAYER1] ||
                    board1.tiles[board::PLAYER2] != board2.tiles[board::PLAYER2] ||
                    board1.turn != board2.turn
                );
            }
        }
    }
}