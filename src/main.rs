extern crate rand;

#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::Write;
use rand::Rng;

include!(concat!(env!("OUT_DIR"), "/lookup.rs"));

mod board;
mod minimax;
mod alphabeta;
mod mcts;
mod supervised;

fn main() {
    let board = board::Board { tiles: [0, 0], turn: rand::thread_rng().gen_range(0, 2) };

    let mut net = supervised::SupervisedNetwork::init();
    net.train();
    net.test();
}

fn play_mcts_minimax(mut board: board::Board, n: usize) {
    let mut mcts_wins = 0;
    let mut minimax_wins = 0;
    let mut draws = 0;

    println!("{}", n);

    for i in 0..n {
        board.tiles = [0, 0];
        board.turn = rand::thread_rng().gen_range(0, 2);

        while board.state() == board::State::Unfinished {
            if board.turn == board::PLAYER1 {
                let (_, m) = mcts::mcts(&board, Some(10000));
                board.make(m);
            } else {
                let (_, m) = minimax::minimax(&board);
                board.make(m);
            }
        }

        match board.state() {
            board::State::Player1Won => mcts_wins += 1,
            board::State::Player2Won => minimax_wins += 1,
            board::State::Draw => draws += 1,
            _ => panic!("Unfinished state after the game ended.")
        }

        if i % 10 == 9 {
            println!("MCTS vs Minimax: {} - {} - {}\n", mcts_wins, minimax_wins, draws);
        }
    }

    assert_eq!(mcts_wins + minimax_wins + draws, n);
}

fn play_human_alphabeta(mut board: board::Board) {
    while board.state() == board::State::Unfinished {

        if board.turn == board::PLAYER2 {
            let (_, m) = alphabeta::alphabeta(&board, -2, 2);
            board.make(m);
            board.print();
        } else {
            let m: usize = get_input();
            board.make(m);
        }
    }

    match board.state() {
        board::State::Player1Won => println!("X won"),
        board::State::Player2Won => println!("O won"),
        board::State::Draw => println!("Draw"),
        _ => panic!("Unfinished state after the game ended.")
    }
}

/*
fn play_human_nn(mut board: board::Board) {
    let mut net = NN::new(&[9, 9, 1]);

    supervised::train_network(&net);

    while board.state() == board::State::Unfinished {

        if board.turn == board::PLAYER2 {
            let (_, m) = net.run(board.split().to_vec());
            board.make(m);
            board.print();
        } else {
            let m: usize = get_input();
            board.make(m);
        }
    }

    match board.state() {
        board::State::Player1Won => println!("X won"),
        board::State::Player2Won => println!("O won"),
        board::State::Draw => println!("Draw"),
        _ => panic!("Unfinished state after the game ended.")
    }
}
*/

fn get_input() -> usize {
    let mut input = String::new();
    let m: usize;

    print!("Enter move: ");
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            m = input.trim().parse::<usize>().unwrap();
        }
        Err(error) => panic!("Error: {}", error),
    }

    assert!(m >= 1 && m <= N2);

    return m;
}
