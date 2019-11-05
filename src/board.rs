extern crate rand;
use rand::Rng;

include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

pub const FULL: u64 = !0_u64 >> (64 - N2);
pub const PLAYER1: usize = 0;
pub const PLAYER2: usize = 1;

#[derive(Eq, PartialEq)]
pub enum State {
    Player1Won,
    Player2Won,
    Draw,
    Unfinished
}

#[derive(Clone, Copy, PartialEq)]
pub struct Board {
    pub tiles: [u64; 2],
    pub turn: usize
}

impl Board {
    pub fn make(&mut self, index: usize) {
        assert!(index >= 1 && index <= N2);

        self.tiles[self.turn] ^= 1 << (index - 1);
        self.turn ^= 1;
    }

    pub fn inverse(&self) -> Board {
        return Board { tiles: [self.tiles[PLAYER2], self.tiles[PLAYER1]], turn: self.turn ^ 1 };
    }

    // Splits the board into an array of floats.
    // 1 for our tiles, -1 for our opponent's, and 0 for the empty ones. 
    pub fn to_binary(&self) -> [f64; N2*3] {
        let mut data: [f64; N2*3] = [0.0; N2*3];

        for i in 0..N2 {
            let square = 1 << i;

            if square & self.tiles[self.turn] != 0 {
                data[i] = 1.0;
            } else if square & self.tiles[self.turn ^ 1] != 0 {
                data[N2+i] = 1.0;
            } else {
                data[N2*2+i] = 1.0;
            }
        }

        return data;
    }

    pub fn gen_moves(&self) -> Vec<usize> {
        let mut moves = Vec::new();
        let empty: u64 = FULL & !(self.tiles[PLAYER1] | self.tiles[PLAYER2]);

        assert!(empty != 0);

        for sqr in 0..N2 {
            let bb = 1 << sqr;

            if bb & empty != 0 {
                moves.push(sqr + 1);
            }
        }

        assert!(moves.len() > 0);

        moves
    }

    pub fn print(&self) {
        println!();
        
        for i in 0..N2 {
            let n: u64 = 1 << i;

            if self.tiles[0] & n != 0 {
                print!("X  ");
            } else if self.tiles[1] & n != 0 {
                print!("O  ");
            } else {
                print!(".  ");
            }

            if i % N == N-1 {
                println!();
            }
        }

        println!();
    }

    pub fn state(&self) -> State {
        for comb in WINNING_COMBS.iter() {
            if self.tiles[PLAYER1] & comb == *comb {
                return State::Player1Won;
            } else if self.tiles[PLAYER2] & comb == *comb {
                return State::Player2Won;
            }
        }

        let n_player1 = self.tiles[PLAYER1].count_ones();
        let n_player2 = self.tiles[PLAYER2].count_ones();

        assert!(n_player1 + n_player2 <= N2 as u32);

        if n_player1 + n_player2 == N2 as u32 {
            return State::Draw;
        }

        State::Unfinished
    }

    pub fn score(&self) -> isize {
        for comb in WINNING_COMBS.iter() {
            if self.tiles[self.turn] & comb == *comb {
                return 1;
            } else if self.tiles[self.turn ^ 1] & comb == *comb {
                return -1;
            }
        }

        0
    }

    pub fn random_move(&self) -> usize {
        let moves = self.gen_moves();
        assert!(moves.len() > 0);

        let pick: usize = rand::thread_rng().gen_range(0, moves.len()) as usize;
        moves[pick]
    }
}

