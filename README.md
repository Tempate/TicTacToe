# TicTacToe

This project is a first attempt at machine learning. In it I've written implementations of minimax, alphabeta, and mcts to play the game. Then, I've trained a NN with supervised learning.

All comments and feedback are appreciated.

## Machine Learning

### Supervised Learning

The current approach is simple: generate all possible boards and train the network once for each of them.

## Dependencies

I am currently using [RustNN](https://github.com/jackm321/RustNN) to deal with the network's internal architecture.

## Running the code

> Requires cargo to be installed.

The code can be run with the command:

```
cargo run
```

However, it's recommended for it to be run with:

```
RUSTFLAGS="$RUSTFLAGS -A dead_code -A unused_variables" cargo run
```
