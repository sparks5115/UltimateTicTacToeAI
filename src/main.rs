mod structs;
mod helpers;

use std::fs::read_to_string;
use std::cmp::{max, min};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::thread::{current, sleep};
use std::time::{Duration, Instant};
use structs::Board;
use crate::helpers::write_to_move_file;
use crate::structs::{Moove, TreeNode};

//TODO remove all prints


pub const TEAM_NAME:&str = "wombat";
pub const TIME_LIMIT:Duration = Duration::from_secs(10);

pub fn main() {
    println!("Hello, my name is {}", TEAM_NAME);
    // initialize board
    let mut board = Board::initialize();
    //board.print();
    loop {


        println!("Waiting for our turn...");

        let mut temp = read_to_string(TEAM_NAME.to_owned() + ".go");
        while temp.is_err() { //block until it finds the file
            temp = read_to_string(TEAM_NAME.to_owned() + ".go");
        }//once this breaks, we have found our file and it is our turn
        println!("found {}.go", TEAM_NAME);

        if read_to_string("end_game").is_ok() {
            sleep(Duration::from_millis(100));
            println!("{}", read_to_string("end_game").expect("please dont panic now"));
            break;
        }
        //let move_file_res = read_to_string("move_file");
        let move_file_str = match read_to_string("move_file") {
            Ok(mv) => { mv }
            Err(_) => { panic!("HONEEEYYYY, WHERE IS MY MOVE FILE???") }
        };

        let (send_best_move, receive_best_move) = mpsc::channel::<Moove>();
        if move_file_str == "" { //this is the first (technically fifth) move
            calculate_best_move(board, send_best_move);
        } else {
            board = board.place_move(Moove::parse_from_string(move_file_str));
            calculate_best_move(board, send_best_move);
        }
        board = board.place_move(receive_best_move.recv().unwrap());
        println!("Move has been placed:");
        //board.print();
    }
}

pub fn calculate_best_move(mut board: Board, send_best_move: Sender<Moove>) {
    //println!("in calculate best move");
    let (send_move, receive_move) = mpsc::channel::<Moove>();
    let (send_kill, receive_kill) = mpsc::channel::<bool>();

    let timer_handler = thread::spawn(move || {
        let start = Instant::now();
        let time_to_wait = Duration::from_millis((TIME_LIMIT.as_millis() - 500) as u64);
        //println!("Timer: waiting for {} seconds", time_to_wait.as_secs());
        let mut best_so_far: Moove = Moove::null();
        while start.elapsed() < time_to_wait { //tries to receive new moves until it needs to submit
            match receive_move.try_recv() {
                Ok(mv) => {
                    //println!("Timer: NEW BEST MOVE JUST DROPPED:::{}", mv.to_string());
                    best_so_far = mv.clone();
                }
                Err(e) => {
                    match e {
                        TryRecvError::Empty => {}
                        TryRecvError::Disconnected => {
                            println!("main thread terminated the connection"); //todo make last move not take full time
                            break; //the main thread has disconnected for some reason (possibly finished all calculations)
                        }
                    }
                }
            }
        }
        //println!("Timer: sending kill message");
        send_kill.send(true).unwrap();

        println!("Timer: submitting move {}, {} {}", TEAM_NAME, best_so_far.big_board, best_so_far.small_board);
        write_to_move_file(best_so_far);
        send_best_move.send(best_so_far).unwrap();
        sleep(Duration::from_secs(1));
    });

    //println!("about to call depth limited");
    depth_limited(&board, send_move, receive_kill);
    timer_handler.join().unwrap();
}

pub fn depth_limited(board: &Board, send_move: Sender<Moove>, receive_kill: Receiver<bool>){
    //println!("in depth limited");
    let mut depth = 2;
    let alpha = i32::MIN;
    let beta = i32::MAX;

    loop {
        //println!("Switching to secret hyper-jets! (depth {})", depth);


        // get value at that depth
        println!("Calling Minimax with depth {}", depth);
        let (mv, h) = minimax(true, depth, alpha, beta, &mut TreeNode::new(board.clone()), &receive_kill);
        if h == -1 { //minimax returned due to the kill message
            break;
        }
        send_move.send(mv).unwrap();

        // iterate depth
        depth += 1;
    }
}


fn minimax(maximizing_player: bool, depth: i32, mut alpha: i32, mut beta: i32, node: &mut TreeNode, receive_kill: &Receiver<bool>) -> (Moove, i32) {

    if receive_kill.try_recv().is_ok() {
        return  (Moove::null(), -1) //this means that we want to kill the recursion
    }
    // if depth == 0 or terminal node
    if (depth == 0) || (node.board.is_winning_or_losing(None) != 0) {
        //println!("depth == 0 or terminal node");
        node.heuristic_value = node.board.get_heuristic_value();
        return (node.board.last_move, node.heuristic_value);
    }
    else if maximizing_player {
        let mut max_eva = i32::MIN;
        let mut best_move = (Moove::null(), i32::MIN);

        // loop through child nodes
        node.children = node.find_all_children();
        for child in node.children.iter_mut() { //for mut child in &mut node.children {
            let (_last_move, hval) = minimax(!maximizing_player, (depth - 1), alpha, beta, child, receive_kill);
            if hval == -1{
                return  (_last_move, hval) //this means that we want to kill the recursion
            }
            node.heuristic_value = hval;
            if best_move.1 < node.heuristic_value {
                best_move = (child.board.last_move, node.heuristic_value)
            }

            max_eva = max(max_eva, hval);
            alpha = max(alpha, max_eva);
            //beta = hval;

            if beta <= alpha {
                break;
            }

        }

        return best_move;
    } else {
        //println!("in not maximizing player");
        let mut min_eva = i32::MAX;
        let mut best_move = (Moove::null(), i32::MAX);

        // loop through child nodes
        node.children = node.find_all_children();
        //node.children.iter().nth(0).unwrap().board.print();
        for child in &mut node.children {
            let (_last_move, hval) = minimax(!maximizing_player, (depth - 1), alpha, beta, child, receive_kill); ;
            if hval == -1{
                return  (_last_move, hval) //this means that we want to kill the recursion
            }
            node.heuristic_value = hval;
            //best_value = min(best_value, node.heuristic_value);
            if best_move.1 > node.heuristic_value {
                best_move = (child.board.last_move, node.heuristic_value)
            }

            min_eva = min(min_eva, hval);
            beta = min(beta, hval);
            //alpha = hval;

            if beta <= alpha {
                break;
            }

            //println!("Total depth: {}, Depth: {}", total_depth, depth);

            //if beta <= alpha { break; }
        }
        return best_move;
    }

}

