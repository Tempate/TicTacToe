use crate::board::Board;

pub trait Player {
    fn best_move(&self, board: &Board) -> usize;
}