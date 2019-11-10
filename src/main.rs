extern crate rand;
use rand::Rng;

#[macro_use]
extern crate lazy_static;

mod board;
mod players;
mod constants;
mod bitboards;

use board::*;
use players::*;

fn main() {
    let player1 = mcts::MCTS{n: 10};
    let mut player2 = networks::supervised::SupervisedNetwork::init();

    train_network(&mut player2);
    //play_match(player1, player2, 10);
}

fn train_network<T>(net: &mut T) where T: networks::network::Network {
    net.train();
    net.test();
}

fn play_match<T, K>(player1: T, player2: K, n: usize) where T: player::Player, K: player::Player {
    let mut player1_wins = 0;
    let mut player2_wins = 0;

    for _ in 0..n {
        match play_game(&player1, &player2) {
            1  => player1_wins += 1,
            -1 => player2_wins += 1,
            _  => ()
        }
    }

    let draws = n - player1_wins - player2_wins;
    println!("{} - {} - {}", player1_wins, player2_wins, draws);
}

fn play_game<T, K>(player1: &T, player2: &K) -> isize where T: player::Player, K: player::Player {
    let mut board = Board { tiles: [0, 0], turn: rand::thread_rng().gen_range(0, 2) };

    while board.state() == State::Unfinished {
        let move_;

        if board.turn == PLAYER1 {
            move_ = player1.best_move(&board);
        } else {
            move_ = player2.best_move(&board);
        }

        board.make(move_);
    }

    match board.state() {
        State::Player1Won => 1,
        State::Player2Won => -1,
        State::Draw => 0,
        _ => panic!("Unfinished state after the game ended.")
    }
}
