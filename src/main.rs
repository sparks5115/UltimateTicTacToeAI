mod structs;
mod helpers;

use std::cmp::{max, min};
use std::fs::read_to_string;
use std::thread;
use std::time::{Duration, Instant};
use structs::Board;
use crate::structs::TreeNode;


pub const TEAM_NAME:&str = "TEMP"; //TODO come up with real team name
pub const TIME_LIMIT:u8 = 10;

pub fn main() {
    // initialize board
    let mut board = Board::initialize();
    board.print();
    println!("{}", board.get_heuristic_value());

    if board.our_turn() {//it is already our turn
        calculate_best_move(&board);

    } else { // wait for turn
        let mut temp = read_to_string(TEAM_NAME.to_owned() + ".go");
        while temp.is_err() { //block until it finds the file
            temp = read_to_string(TEAM_NAME.to_owned() + ".go");
        }//once this breaks, we have found our file
        calculate_best_move(&board);
    }
}

pub fn calculate_best_move(board: &Board) {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn depth_limited(board: &Board){
// set time number (?) / record time
    let now = Instant::now();
//check if end_game exists; if so, gameWon = true and break;
//if "endGame exists" { break; } //TODO make this work

// read in move_file

// determine step (Moove)
// While timer isn't done:
    let mut depth = 0;
    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    loop {
// get value at that depth
        let value = minimax(true, depth, alpha, beta, TreeNode::new(board));

// give value to timer thread

// iterate depth
        depth += 1;

// if timer thread = finished (read a message sent by the thread?)
        break;
    }

// write to move_file
    let elapsed_time = now.elapsed();
// note/print time?
//println!("Moove took {} seconds.", elapsed_time.as_secs());
}


fn minimax(maximizing_player: bool, depth: i32, mut alpha: i32, mut beta: i32, mut node: TreeNode) -> i32 {

    // if depth == 0 or terminal node
    if (depth == 0) || (node.board.is_winning_or_losing(None) != 0) {
        //evaluate heuristic and return
    }

    if maximizing_player {
        let mut best_value = i32::MIN;

        // loop through child nodes
        for child in node.children {
            node.heuristic_value = minimax(!maximizing_player, depth - 1, alpha, beta, child);
            best_value = i32::max(best_value, node.heuristic_value);
            alpha = max(alpha, best_value);
            if beta <= alpha { break; }
        }
        return best_value;
    } else {
        let mut best_value = i32::MAX;

        // loop through child nodes
        for child in node.children {
            node.heuristic_value = minimax(!maximizing_player, depth - 1, alpha, beta, child);
            best_value = min(best_value, node.heuristic_value);
            if beta <= alpha { break; }
        }
        return best_value;
    }

}

