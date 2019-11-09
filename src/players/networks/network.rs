use crate::board::Board;
use crate::players::player::Player;

pub trait Network {
    fn train(&mut self);
    fn test(&self);

    fn play(&self, board: &Board) -> usize;
}

impl<T> Player for T where T: Network {
    fn best_move(&self, board: &Board) -> usize {
        self.play(board)
    }
}