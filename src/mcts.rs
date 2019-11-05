extern crate rand;
use rand::Rng;

use crate::board;

struct Node {
    board: board::Board,
    move_: usize,

    reward: f64,
    visits: f64,
}

pub fn mcts(board: &board::Board, n: Option<usize>) -> (isize, usize) {
    let m = n.unwrap_or(1000);

    let copy = board.clone();
    let mut root = Node { board: copy, move_: 0, reward: 0.0, visits: 0.0 };
    let mut children: Vec<Node> = root.expand();

    for _ in 0..m {
        let cp: f64 = 0.5_f64.sqrt();
        let node: &mut Node = root.tree_policy(&mut children, cp);

        root.visits += 1.0;
        node.random_rollout();
    }

    return (0, root.tree_policy(&mut children, 0.0).move_);
}

impl Node {
    pub fn expand(&self) -> Vec<Node> {
        let moves: Vec<usize> = self.board.gen_moves();
        let mut nodes: Vec<Node> = Vec::with_capacity(moves.len());

        for m in moves {
            let mut copy = self.board.clone();
            copy.make(m);

            nodes.push( Node { board: copy, move_: m, reward: 0.0, visits: 0.0 } );
        }

        return nodes;
    }

    pub fn tree_policy<'a>(&self, children: &'a mut Vec<Node>, cp: f64) -> &'a mut Node {
        let mut max_score: f64 = std::f64::MIN;
        let mut best_nodes: Vec<&mut Node> = Vec::with_capacity(children.len());

        for child in children {
            let score = self.ucb_score(child, cp);

            if score > max_score {
                max_score = score;
                best_nodes.clear();
                best_nodes.push(child);
            } else if score == max_score {
                best_nodes.push(child);
            }
        }

        assert!(best_nodes.len() > 0);

        let pick: usize = rand::thread_rng().gen_range(0, best_nodes.len()) as usize;
        best_nodes.remove(pick)
    }

    pub fn random_rollout(&mut self) {
        let mut copy = self.board.clone();
        let mut state = copy.state();

        while state == board::State::Unfinished {
            let m: usize = copy.random_move();
            copy.make(m);

            state = copy.state();
        }

        self.visits += 1.0;

        match state {
            board::State::Draw => self.reward += 0.5,
            board::State::Unfinished => panic!("Unfinished random rollout."),
            _ => {
                if self.board.turn == copy.turn {
                    self.reward += 1.0
                }
            }
        }
    }

    fn ucb_score(&self, child: &mut Node, cp: f64) -> f64 {
        if child.visits == 0.0 {
            return std::f64::MAX;
        }
        
        let exploitation = child.reward / child.visits;
        let exploration = 2.0 * cp * (2.0 * self.visits.ln() / child.visits).sqrt();
        
        exploitation + exploration
    }
}
