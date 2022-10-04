mod structs;
mod helpers;

use std::fs::read_to_string;
use std::cmp::{max, min};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};
use std::thread;
use std::thread::{current, sleep};
use std::time::{Duration, Instant};
use structs::Board;
use crate::helpers::write_to_move_file;
use crate::structs::{Moove, TreeNode};


pub const TEAM_NAME:&str = "TEMP"; //TODO come up with real team name
pub const TIME_LIMIT:Duration = Duration::from_secs(10);
//static mut NEXT_MOVE:Moove = Moove::null();
//static mut BEST_HEURISTIC: i32 = i32::MIN;

pub fn main() {
    // initialize board
    let mut board = Board::initialize();
    board.print();
    //println!("{}", board.get_heuristic_value());

    println!("Waiting for our turn...");
    loop {
        let mut temp = read_to_string(TEAM_NAME.to_owned() + ".go");
        while temp.is_err() { //block until it finds the file
            temp = read_to_string(TEAM_NAME.to_owned() + ".go");
        }//once this breaks, we have found our file and it is our turn
        //let move_file_res = read_to_string("move_file");
        let move_file_str = match read_to_string("move_file") {
            Ok(mv) => {mv}
            Err(_) => {panic!("HONEEEYYYY, WHERE IS MY MOVE FILE???")}
        };

        // check if end_game file is there; if yes, break
        if read_to_string("end_game").is_ok() {
            //break; TODO remove this being a comment later
        }

        if move_file_str == ""{ //this is the first (technically fifth) move
            calculate_best_move(board);
        }else{
            board = board.place_move(Moove::parse_from_string(move_file_str));
            calculate_best_move(board);
        }
        break; //todo
    }
}

pub fn calculate_best_move(board: Board) {
    println!("in calculate best move");
    let (send_move, receive_move) = mpsc::channel::<Moove>();
    let (send_kill, receive_kill) = mpsc::channel::<bool>(); //TODO KILLLLL

    let timer_handler = thread::spawn(move || {
        let start = Instant::now();
        let time_to_wait = Duration::from_secs(TIME_LIMIT.as_secs() - 1);//TODO give this closer time
        //println!("Timer: waiting for {} seconds", time_to_wait.as_secs());
        let mut best_so_far: Moove = Moove::null();
        while start.elapsed() < time_to_wait { //tries to receive new moves until it needs to submit
            match receive_move.try_recv() {
                Ok(mv) => {
                    println!("Timer: NEW BEST MOVE JUST DROPPED:::{}", mv.to_string());
                    best_so_far = mv.clone();
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
        //println!("Timer: sending kill message");
        send_kill.send(true).unwrap();

        //println!("Timer: submitting move");
        write_to_move_file(best_so_far);
        sleep(Duration::from_secs(1)); //todo hone this value with ref
    });

    //println!("about to call depth limited");
    depth_limited(&board, send_move);
    timer_handler.join().unwrap();
}

pub fn depth_limited(board: &Board, send_move: Sender<Moove>){
    //println!("in depth limited");
    let mut depth = 2;
    let alpha = i32::MIN;
    let beta = i32::MAX;

    loop {
        //println!("Switching to secret hyper-jets! (depth {})", depth);


        // get value at that depth
        let (mv, h) = minimax(true, depth, depth, alpha, beta, &mut TreeNode::new(board.clone()));

            send_move.send(mv).unwrap();

        // iterate depth
        depth += 1;

        if depth >= 10 {
            break;
        }
    }
}


fn minimax(maximizing_player: bool, depth: i32, total_depth: i32, alpha: i32, beta: i32, node: &mut TreeNode) -> (Moove, i32) {
    println!("in minimax, depth = {}", depth);

    // if depth == 0 or terminal node
    if (depth == 0) || (node.board.is_winning_or_losing(None) != 0) {
        println!("depth == 0 or terminal node");
        node.heuristic_value = node.board.get_heuristic_value();
        return (node.board.last_move, node.heuristic_value);
    }
    else if maximizing_player {
        //println!("in maximizing player, depth = {}", depth);
        println!("children of node:");
        let mut best_move = (Moove::null(), i32::MIN);

        // loop through child nodes
        node.children = node.find_all_children();
        //node.children.iter().nth(0).unwrap().board.print();
        for child in node.children.iter_mut() { //for mut child in &mut node.children {
            //println!("iterating through children in maximizing player");
            let (_last_move, hval) = minimax(!maximizing_player, (depth - 1), total_depth, alpha, beta, child);
            node.heuristic_value = hval;
            //best_value = i32::max(best_move, node.heuristic_value);
            if best_move.1 < node.heuristic_value {
                best_move = (child.board.last_move, node.heuristic_value)
            }
            if beta <= max(alpha, best_move.1) { break; }
        }

        return best_move;
    } else {
        println!("in not maximizing player");
        let mut best_move = (Moove::null(), i32::MAX);

        // loop through child nodes
        node.children = node.find_all_children();
        //node.children.iter().nth(0).unwrap().board.print();
        for child in &mut node.children {
            let (_garbage, hval) = minimax(!maximizing_player, (depth - 1), total_depth, alpha, beta, child);
            node.heuristic_value = hval;
            //best_value = min(best_value, node.heuristic_value);
            if best_move.1 > node.heuristic_value {
                best_move = (child.board.last_move, node.heuristic_value)
            }

            //println!("Total depth: {}, Depth: {}", total_depth, depth);

            if beta <= alpha { break; }
        }
        return best_move;
    }

}

