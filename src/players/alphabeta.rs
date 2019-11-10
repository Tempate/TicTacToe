use crate::constants::*;
use crate::players::player::Player;
use crate::board::{Board, State};

pub struct AlphaBeta;

impl AlphaBeta {
    fn search(board: &Board, mut alpha: isize, beta: isize) -> (isize, usize) {
        if board.state() != State::Unfinished {
            // The game has ended so there is no best move.
            return (board.score(), 0);
        }
        
        let mut moves: Vec<usize> = board.gen_moves();
        assert!(moves.len() > 0);

        let mut max_score: isize = -2;
        let mut best_move: usize = 0;

        // Forced moves are precalculated to speed up the search.
        let forced_move = board.find_forced();

        if forced_move != 0 {
            // The move is not returned instantly because the score
            // needs to be figured out.
            moves.clear();
            moves.push(forced_move);
        }

        for move_ in moves {
            let mut copy = board.clone();
            copy.make(move_);

            let score = -AlphaBeta::search(&copy, -beta, -alpha).0;

            if score > max_score {
                max_score = score;
                best_move = move_;

                // AlphaBeta prunning
                if max_score >= beta {
                    break;
                }

                if max_score > alpha {
                    alpha = max_score;
                }
            }
        }

        assert!(best_move >= 1 && best_move <= N2);
        assert!(max_score > -2);

        return (max_score, best_move);
    }
}

impl Player for AlphaBeta {
    fn best_move(&self, board: &Board) -> usize {
        AlphaBeta::search(board, -2, 2).1
    }
}
