mod structs;
mod helpers;

use std::cmp::{max, min};
use structs::Board;
use crate::structs::TreeNode;


pub const TEAM_NAME:&str = "TEMP"; //TODO come up with real team name

pub fn main() {
    let mut board = Board::initialize();
    board.print();
}

fn minimax(maximizingPlayer: bool, depth: i32, mut alpha: i32, mut beta: i32, node: TreeNode) {
    // if depth == 0 or terminal node
    if (depth == 0) || (0 != 1) {

    }

    if maximizingPlayer {
        let bestValue = -inf;

        // loop through child nodes
        for child in node.children {
            let value = minimax (!maximizingPlayer, depth-1, alpha, beta, child);
            bestValue = max(bestValue, value);
            alpha = max(alpha, bestValue);
            if beta <= alpha { break; }
        }

        return bestValue;
    }

    if !maximizingPlayer {
        let bestValue = inf;

        // loop through child nodes
        for child in node.children {
            let value = minimax (!maximizingPlayer, depth-1, alpha, beta, child);
            bestValue = min(bestValue, value);
            if beta <= alpha { break; }
        }
        return bestValue;
    }

}