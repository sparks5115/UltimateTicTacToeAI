mod structs;
mod helpers;

use std::fs::read_to_string;
use std::cmp::{max, min};
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::thread::{current, sleep};
use std::time::{Duration, Instant};
use structs::Board;
use crate::structs::{Moove, TreeNode};


pub const TEAM_NAME:&str = "TEMP"; //TODO come up with real team name
pub const TIME_LIMIT:Duration = Duration::from_secs(10);

pub fn main() {
    // initialize board
    let board = Board::initialize();
    board.print();
    println!("{}", board.get_heuristic_value());

    if board.our_turn() {//it is already our turn
        calculate_best_move(board);

    } else { // wait for turn
        loop {
            let mut temp = read_to_string(TEAM_NAME.to_owned() + ".go");
            while temp.is_err() { //block until it finds the file
                temp = read_to_string(TEAM_NAME.to_owned() + ".go");
            }//once this breaks, we have found our file

            // check if end_game file is there; if yes, break
            if read_to_string("end_game").is_ok() {
                break;
            }

            calculate_best_move(board);
        }
    }
}

pub fn calculate_best_move(board: Board) {
    let (send_move, receive_move) = mpsc::channel::<Moove>();
    let (send_kill, receive_kill) = mpsc::channel::<bool>();

    let timer_handler = thread::spawn(move || {
        let start = Instant::now();
        let time_to_wait = Duration::from_secs(TIME_LIMIT.as_secs() - 1);//TODO give this closer time
        println!("Timer: waiting for {} seconds", time_to_wait.as_secs());
        let mut best_so_far: Moove;
        while start.elapsed() < time_to_wait { //tries to receive new moves until it needs to submit
            match receive_move.try_recv() {
                Ok(mv) => {
                    println!("{}", mv.to_string());
                    best_so_far = mv;
                }
                Err(e) => {match e {
                    TryRecvError::Empty => {}
                    TryRecvError::Disconnected => {
                        println!("main thread terminated the connection");
                        break; //the main thread has disconnected for some reason (possibly finished all calculations)
                    }
                }}
            }
        }
        println!("Timer: sending kill message");
        send_kill.send(true).unwrap();

        println!("Timer: submitting move");
        //TODO submit the move
    });

    for i in  0..5{
        send_move.send(depth_limited(&board)).unwrap();
        sleep(Duration::from_secs(1));
    }
    timer_handler.join().unwrap();
}

pub fn depth_limited(board: &Board) -> Moove{
    let mut depth = 0;
    let alpha = i32::MIN;
    let beta = i32::MAX;

    loop {
        // get value at that depth
        let value = minimax(true, depth, depth, alpha, beta, TreeNode::new(board.clone()));

        // iterate depth
        depth += 1;

        // if timer thread = finished (read a message sent by the thread?)
        break;
    }

// write to move_file
// note/print time?
    return Moove::null();
}


fn minimax(maximizing_player: bool, depth: i32, total_depth: i32, mut alpha: i32, mut beta: i32, mut node: TreeNode) -> i32 {

    // if depth == 0 or terminal node
    if (depth == 0) || (node.board.is_winning_or_losing(None) != 0) {
        node.heuristic_value = node.board.get_heuristic_value();
        return node.heuristic_value;
    }
    else if maximizing_player {
        let mut best_value = i32::MIN;

        let best_move = Moove::null();
        // loop through child nodes
        for child in node.children {
            node.heuristic_value = minimax(!maximizing_player, depth - 1, total_depth, alpha, beta, child);
            best_value = i32::max(best_value, node.heuristic_value);
            if total_depth-1 == depth {

            }
            alpha = max(alpha, best_value);
            if beta <= alpha { break; }
        }

        return best_value;
    } else {
        let mut best_value = i32::MAX;

        // loop through child nodes
        for child in node.children {
            node.heuristic_value = minimax(!maximizing_player, depth - 1, total_depth, alpha, beta, child);
            best_value = min(best_value, node.heuristic_value);
            if beta <= alpha { break; }
        }
        return best_value;
    }

}

