use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod constants;
use constants::*;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("lookup.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(format!("const N: usize = {};", N).as_bytes()).unwrap();
    f.write_all(format!("const N2: usize = {};", N2).as_bytes()).unwrap();
    f.write_all(format!("const N_COMBS: usize = {};", N_COMBS).as_bytes()).unwrap();

    let winning_combs_str = format!("const WINNING_COMBS: [u64; N_COMBS] = {:?};", gen_winning_combs());

    f.write_all(winning_combs_str.as_bytes()).unwrap();
}

fn gen_winning_combs() -> [u64; N_COMBS] {
    // All the possible n-in-a-row combinations.
    let mut winning_combs: [u64; N_COMBS] = [0; N_COMBS];
    
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
        winning_combs[i] = ROW << (i * N);
        winning_combs[N+i] = col << i;
    }

    winning_combs[N_COMBS-2] = prin_diag;
    winning_combs[N_COMBS-1] = anti_diag;

    winning_combs
}
