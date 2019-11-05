use crate::board;

include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

pub fn minimax(board: &board::Board) -> (isize, usize) {
    if board.state() != board::State::Unfinished {
        return (board.score(), 0);
    }
    
    let moves: Vec<usize> = board.gen_moves();
    assert!(moves.len() > 0);

    let mut max_score: isize = -2;
    let mut best_move: usize = 0;

    for move_ in moves {
        let mut copy = board.clone();
        copy.make(move_);

        let (mut score, _) = minimax(&copy);
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