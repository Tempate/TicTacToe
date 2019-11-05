use crate::board;

include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

pub fn alphabeta(board: &board::Board, mut alpha: isize, beta: isize) -> (isize, usize) {
    if board.state() != board::State::Unfinished {
        // The game has ended so there is no best move.
        return (board.score(), 0);
    }
    
    let mut moves: Vec<usize> = board.gen_moves();
    assert!(moves.len() > 0);

    let mut max_score: isize = -2;
    let mut best_move: usize = 0;

    // Forced moves are precalculated to speed up the search.
    let forced_move = find_forced(board);

    if forced_move != 0 {
        // The move is not returned instantly because the score
        // needs to be figured out.
        moves.clear();
        moves.push(forced_move);
    }

    for move_ in moves {
        let mut copy = board.clone();
        copy.make(move_);

        let (mut score, _) = alphabeta(&copy, -beta, -alpha);
        score = -score;

        if score > max_score {
            max_score = score;
            best_move = move_;

            // Alpha-Beta prunning
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

// Calculates moves that produce an instant win and 
// moves that prevent the opponent from winning instantly.
fn find_forced(board: &board::Board) -> usize {
    const MAX: u32 = (N - 1) as u32;

    let empty: u64 = board::FULL & !(board.tiles[board::PLAYER1] | board.tiles[board::PLAYER2]);
    let mut forced_move: usize = 0;

    for comb in WINNING_COMBS.iter() {
        if empty & comb != 0 {
            if (board.tiles[board.turn] & comb).count_ones() == MAX {
                return (empty & comb).trailing_zeros() as usize + 1;
            } 
            
            // A blocking move will only be played if there're no winning moves.
            else if (board.tiles[board.turn ^ 1] & comb).count_ones() == MAX {
                forced_move = (empty & comb).trailing_zeros() as usize + 1;
            }
        }
    }

    forced_move
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_forced() {
        let tests = [
            (board::Board { tiles: [0b100010100, 0b101001], turn: 0 }, 7),
            (board::Board { tiles: [0b11000, 0b1], turn: 0 }, 6),
            (board::Board { tiles: [0b100000000, 0b101], turn: 0 }, 2),
            (board::Board { tiles: [0b1010001, 0b101110], turn: 0 }, 9),
            (board::Board { tiles: [0b10001, 0b1000000], turn: 0 }, 9)
        ];

        for test in tests.iter() {
            // Every test is performed for both turns to make sure it can 
            // block winning moves and make winning moves.
            let mut clone = test.0.clone();
            clone.turn ^= 1;
            
            assert_eq!(find_forced(&test.0), test.1);
            assert_eq!(find_forced(&clone), test.1);
        }
    }
}