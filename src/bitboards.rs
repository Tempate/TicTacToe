use crate::constants::*;

lazy_static! {
    pub static ref WINNING_STATES: [u64; N_STATES] = gen_winning_states();
}

fn gen_winning_states() -> [u64; N_STATES] {
    // All the possible n-in-a-row combinations.
    let mut winning_states: [u64; N_STATES] = [0; N_STATES];
    
    const ROW: u64 = (1 << N) - 1;

    let mut col: u64 = 0;
    let mut prin_diag: u64 = 0;
    let mut anti_diag: u64 = 0;

    // Generate the column, principal diagonal and anti-diagonal's bitboards.
    for _ in 0..N {
        col <<= N;
        col += 1;

        prin_diag <<= N + 1;
        prin_diag += 1;

        anti_diag += 1;
        anti_diag <<= N - 1;
    }

    for i in 0..N {
        winning_states[i] = ROW << (i * N);
        winning_states[N+i] = col << i;
    }

    winning_states[N_STATES-2] = prin_diag;
    winning_states[N_STATES-1] = anti_diag;

    winning_states
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_bbs() {
        let board = Board { tiles: [0, 0], turn : 0};

        for i in 0..N {
            let mut bb = WINNING_STATES[i];

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
            let mut bb = WINNING_STATES[i];

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
        let mut bb = WINNING_STATES[2*N];

        assert_eq!(bb.count_ones(), N as u32);

        while bb != 0 {
            let lowest_bit = bb.trailing_zeros() as usize;
            assert_eq!(lowest_bit % N, lowest_bit / N);
            bb &= bb - 1;
        }
    }

    #[test]
    fn diag2_bbs() {
        let mut bb = WINNING_STATES[2*N+1];

        assert_eq!(bb.count_ones(), N as u32);

        while bb != 0 {
            let lowest_bit = bb.trailing_zeros() as usize;
            assert_eq!(lowest_bit % N, N - (lowest_bit / N) - 1);
            bb &= bb - 1;
        }
    }
}