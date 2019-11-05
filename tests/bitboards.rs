include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

use TicTacToe;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_bbs() {
        let board = TicTacToe::Board { tiles: [0, 0], turn : 0};

        for i in 0..N {
            let mut bb = WINNING_COMBS[i];

            assert_eq!(bb.count_ones(), N as u32);

            while bb != 0 {
                let lowest_bit = bb.trailing_zeros() as usize;
                assert_eq!(lowest_bit / N, i);
                bb &= bb - 1;
            }
        }        
    }

    #[test]
    fn column_bbs() {
        for i in N..2*N {
            let mut bb = WINNING_COMBS[i];

            assert_eq!(bb.count_ones(), N as u32);

            while bb != 0 {
                let lowest_bit = bb.trailing_zeros() as usize;
                assert_eq!(lowest_bit % N, i % N);
                bb &= bb - 1;
            }
        }
    }

    #[test]
    fn diag1_bbs() {
        let mut bb = WINNING_COMBS[2*N];

        assert_eq!(bb.count_ones(), N as u32);

        while bb != 0 {
            let lowest_bit = bb.trailing_zeros() as usize;
            assert_eq!(lowest_bit % N, lowest_bit / N);
            bb &= bb - 1;
        }
    }

    #[test]
    fn diag2_bbs() {
        let mut bb = WINNING_COMBS[2*N+1];

        assert_eq!(bb.count_ones(), N as u32);

        while bb != 0 {
            let lowest_bit = bb.trailing_zeros() as usize;
            assert_eq!(lowest_bit % N, N - (lowest_bit / N) - 1);
            bb &= bb - 1;
        }
    }
}