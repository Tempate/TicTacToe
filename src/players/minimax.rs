use crate::players::player::Player;
use crate::board::{Board, State};

include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

pub struct Minimax;

impl Minimax {
    fn search(board: &Board) -> (isize, usize) {
        if board.state() != State::Unfinished {
            return (board.score(), 0);
        }
        
        let moves: Vec<usize> = board.gen_moves();
        assert!(moves.len() > 0);

        let mut max_score: isize = -2;
        let mut best_move: usize = 0;

        for move_ in moves {
            let mut copy = board.clone();
            copy.make(move_);

            let (mut score, _) = Minimax::search(&copy);
            score = -score;

            if score > max_score {
                max_score = score;
                best_move = move_;
            }
        }

        assert!(best_move >= 1 && best_move <= N2);
        assert!(max_score > -2);

        return (max_score, best_move);
    }
}

impl Player for Minimax {
    fn best_move(&self, board: &Board) -> usize {
        Minimax::search(board).1
    }
}
