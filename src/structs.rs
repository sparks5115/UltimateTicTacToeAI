use std::fs::read_to_string;
use crate::{helpers};
use crate::helpers::{block_opponent, is_board_won, is_two_in_row_them, is_two_in_row_us};
use super::TEAM_NAME;

///constant that holds our decided upon heuristic values
const HEURISTIC: Heuristic = Heuristic{
    total_win_loss: i32::MAX,
    board_win_loss: 100,
    two_boards_in_row: 200,
    block_opponent_board: 150,
    useless_board_win: -150,
    two_in_row: 5,
    block_opponent: 20,
    useless_move: -20,
};

///holds a move made by either team
#[derive(Copy, Clone)]
pub struct Moove {//like a cow
    pub team: i8,
    pub big_board: u8,
    pub small_board: u8,
}
impl Moove { //implementation? Inside here are functions for the struct
pub const fn null() -> Moove{
    return Moove{
        team: 66,
        big_board: 9,
        small_board: 9
    }

}

    ///parses a string (such as the one in move_file) into a Moove struct
    /// # Examples:
    /// m: Moove = Moove::parse_from_string(read_to_string("move_file"));
    pub fn parse_from_string(move_string: String) -> Moove {

        let v: Vec<&str> = move_string.trim().split(' ').collect(); //this only works if it is formatted correctly

        return Moove::new(v[0].parse().unwrap(),
                          v[1].chars().nth(0).expect("Invalid move_string").to_digit(10).unwrap() as u8,
                          v[2].chars().nth(0).expect("Invalid move_string").to_digit(10).unwrap() as u8)
    }

    pub fn new(team_name:String, big_board: u8, small_board: u8) -> Moove {
        let mut team_num = 66;
        if team_name == TEAM_NAME {
            team_num = 1;
        }else{
            team_num = -1;
        }
        return Moove{
            team: team_num,
            big_board,
            small_board,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.team, self.big_board, self.small_board)
    }
}

///state of the board is stored in an array as follows:
/// big_board number 0 takes up spaces 0-8
/// big_board number 1 takes up spaces 9-15
#[derive(Copy, Clone)]
pub struct Board {
    state: [i8;81],
    pub(crate) last_move: Moove,
}
impl Board {

    fn new() -> Board {
        Board{state: [0; 81], last_move: Moove::null()}
    }

    pub fn initialize() -> Board {

        let mut b = Board::new();

        println!("Waiting for first four moves...");
        let mut ffm_result = read_to_string(r"first_four_moves");
        loop{
            while ffm_result.is_err() || ffm_result.as_ref().unwrap().is_empty() { //block until it finds the file
                ffm_result = read_to_string(r"first_four_moves");
            }
            let mut new_line_chars = 0;
            for char in ffm_result.as_ref().unwrap().chars(){
                if char == '\n'{
                    new_line_chars += 1;
                }
            }
            if new_line_chars >= 3 {break;}
        }
        let mut first_four_moves: String = ffm_result.expect("wohoo"); // becuase you wait until there is not an error teachnically this isn't needed, but it'll freak anyways

        println!("Found first_four_moves.txt: \n {}", first_four_moves);

        //println!("Placing first moves on Board...");
        if first_four_moves.chars().last().unwrap()!='\n'{
            first_four_moves.push('\n');//appends new line if not already there
        }
        let mut line= String::new();
        for char in first_four_moves.chars() {
            if char == '\n' {
                b = b.place_move(Moove::parse_from_string(line.clone()));//because of this clone, this can likely be done async
                line = "".parse().unwrap();
            }else {
                line.push(char);
            }
        }
        return b;
    }

    ///returns true if it is our turn to make a move, false otherwise
    pub fn our_turn(&self) -> bool{
        return self.last_move.team == -1;
    }

    ///Converts from notation of big_board, small_board into the index in Board's state
    pub fn get_index(big_board: &u8, small_board: &u8) -> usize{
        ((big_board*9) + small_board) as usize
    }

    ///called on a board, places a Moove onto its own state
    pub fn place_move(&self, mv:Moove) -> Board {
        //ln!("Placing Move:{} ({}, {})", mv.team, mv.big_board, mv.small_board);

        // create new board
        let mut b: Board = Board { state: self.state.clone(), last_move: mv.clone() };
        b.state[Board::get_index(&mv.big_board, &mv.small_board)] = mv.team;
        return b;
    }

    ///gets the heuristic of the board that it is called on
    pub fn get_heuristic_value(&self) -> i32{
        let big_board_state = self.get_big_board_state();
        let mut h_val = 0;
        let temp = self.is_winning_or_losing(Some(big_board_state)) as i32;
        if temp != 0 {
            return temp * HEURISTIC.total_win_loss; //if the state is winning or losing, there is no need to continue
        }

        h_val += HEURISTIC.board_win_loss * (self.net_boards_won(Some(big_board_state)) as i32);
        h_val += HEURISTIC.two_boards_in_row * (self.net_two_boards_in_row(Some(big_board_state)) as i32);
        h_val += HEURISTIC.block_opponent_board * (self.net_blocked_boards(Some(big_board_state)) as i32);
        //h_val += HEURISTIC.useless_board_win * (self.net_useless_boards(); //todo add useless_board_win
        h_val += HEURISTIC.two_in_row * (self.net_two_in_row() as i32);
        h_val += HEURISTIC.block_opponent * (self.net_blocked() as i32);
        //h_val += HEURISTIC.useless_move * self.net_useless(); //todo add useless_move
        println!("Getting heuristic... {}", h_val);
        return h_val;
    }

    /// calculates number of boards blocked in all directions for us - them
    pub fn net_useless_boards(&self, big_board_state: Option<[i8; 9]>) -> i8{
        let bbs =self.extract_big_board_state(big_board_state);
        return 5;
    }
    
    ///checks if the game is over, and one team has won (checks if it's a terminal node)
    /// # Returns: -1 if opponent has won, 1 if we have won, 0 if not won
    pub fn is_winning_or_losing(&self, big_board_state: Option<[i8; 9]>) -> i8{
        let bbs = match big_board_state {
            None => {self.get_big_board_state()},
            Some(a) => {a},
        };
        is_board_won(&bbs[..])
    }

    ///calculates boards won by us, minus boards won by opponent
    pub fn net_boards_won(&self, big_board_state: Option<[i8; 9]>) -> i8{
        let bbs =self.extract_big_board_state(big_board_state);
        bbs.iter().sum()
    }

    /// calculates how many net blocked rows/cols/diags we did - they did
    pub fn net_blocked_boards(&self, big_board_state: Option<[i8; 9]>) -> i8 {
        let bbs =self.extract_big_board_state(big_board_state);
        return block_opponent(&bbs[..]);
    }

    /// calculates how many net blocked small boards' rows/cols/diagonals there are
    pub fn net_blocked(&self) -> i8 {
        let mut num_blocked = 0;
        for i in 0..9{
            let small = self.get_small_board_state(&(i as u8));
            num_blocked += block_opponent(small);
        }
        return num_blocked;
    }

    ///used to unwrap the big board state from the option<bbs>
    pub fn extract_big_board_state(&self, big_board_state: Option<[i8; 9]>) -> [i8; 9]{
        let bbs = match big_board_state {
            None => {self.get_big_board_state()},
            Some(a) => {a},
        };
        return bbs;
    }
    ///calculates how many sets of 2 in a row there are - how many sets of 2 for opponent
    pub fn net_two_in_row(&self) -> i8 {
        let mut num_two_in_row = 0;
        for i in 0..9{
            let small = self.get_small_board_state(&(i as u8));
            num_two_in_row += is_two_in_row_us(small) - is_two_in_row_them(small);
        }
        return num_two_in_row;
    }
    /// calculates how many sets of 2 big boards in a row there are - how many sets of 2 for opponent
    pub fn net_two_boards_in_row(&self, big_board_state: Option<[i8; 9]>) -> i8 {
        let bbs =self.extract_big_board_state(big_board_state);

        let num_two_boards_in_row_us = is_two_in_row_us(&bbs[..]);
        let num_two_boards_in_row_them = is_two_in_row_them(&bbs[..]);

        return num_two_boards_in_row_us - num_two_boards_in_row_them;
    }

    ///checks the entire board.state and returns an array representing the board as a singular tic tac toe board.
    ///  (if you ignore that the individual games are games and treat them as their results, it treats the big board as a simple tic tac toe game)
    pub fn get_big_board_state(&self) -> [i8; 9]{
        let mut big_board: [i8; 9] = [0 as i8; 9];
        for i in 0..9 {
            big_board[i] = self.board_is_won( &(i as u8));
        }
        return big_board;
    }

    ///wraps is_board_won for convenience such that it takes a board number as an argument instead of an array representing a board
    /// # Returns:
    /// -1 if opponent has won
    /// 1 if we have won
    /// 0 if not won
    pub fn board_is_won(&self, board_number: &u8) -> i8{
        return is_board_won(self.get_small_board_state(board_number));
    }

    ///returns an array of length 9 representing a single small board in the game given a board number
    pub fn get_small_board_state(&self, board_number:&u8) -> &[i8] {
        &self.state[(board_number*9) as usize .. (board_number*9 + 9) as usize]
    }

    pub fn clone_state(&self) -> Board{
        //b:Board = Board::new();
        let board_clone:Board =  Board { state: self.state, last_move: Moove::null() };
        return board_clone;
    }

    ///# for debugging purposes only
    /// prints the board
    pub fn print(&self){
        println!("-------------------------------------");
        for row in 0..3 {
            for subrow in 0..3 {
                for col in 0..3 {
                    for subcol in 0..3 {
                        let rep: String = match self.state[(row * 27) + (subrow * 3) + (col * 9) + subcol]{
                            0=>"..".to_string(),
                            1=>"❌".to_string(),
                            -1=>"〇".to_string(),
                            _=>"😱".to_string(), //this symbol is bad
                        };
                        print!(" {} ", rep);
                    }
                    print!("|")
                }
                println!();
            }println!("-------------------------------------");
        }
    }
}

/// just an easy way to encompass our heuristic in one place
pub struct Heuristic {
    total_win_loss: i32, //game win or loss: three big boards in a row
    board_win_loss: i32, //win or loss on small board
    two_boards_in_row: i32, //win two boards in a row
    block_opponent_board: i32, //block opponent's two in a row
    useless_board_win: i32, //board that is blocked
    two_in_row: i32, //two in a row on small board
    block_opponent: i32, //block on small board
    useless_move: i32, //blocked move on small board
}

///wraps a board and all the information needed to run the minimax algorithm
pub struct TreeNode {
    pub board: Board,
    pub heuristic_value: i32,
    pub children: Vec<TreeNode>
}

impl TreeNode{
    pub fn new(board: Board) -> TreeNode{
        return TreeNode{
            board,
            heuristic_value: 0,
            children: Vec::new(),
        }
    }

    ///builds all of this node's children (a collection of tree nodes denoting the next legal moves that could be made)
    pub fn find_all_children(&self) -> Vec<TreeNode>{
        // initialize all children vector
        let mut all_children: Vec<TreeNode> = Vec::new();

        // find the last move
        let last_move = &self.board.last_move;
        let next_board = &last_move.small_board;

        // if the square of that move is won:
        if self.board.board_is_won(next_board) != 0 {
            // go through all other squares
            for i in 0..9 {
                // if square is not won
                if 0 == self.board.board_is_won(&i) {
                    // go through every space in square
                    for j in 0..9 {
                        // if space is open, add TreeNode to vector
                        if self.board.state[(i*9+j) as usize] == 0 {
                            // calculate new team name
                            let team_name: String;
                            if last_move.team == 1 {
                                team_name = "enemy".parse().unwrap();
                            }else{
                                team_name = TEAM_NAME.parse().unwrap();
                            }
                            // create new move and add to board
                            let new_moove = Moove::new(team_name, i, j); // board + new node
                            let new_node = TreeNode::new(self.board.place_move(new_moove));
                            // push new child
                            all_children.push(new_node);
                        }
                    }
                }

            }
        }
        else {
            // go through every space in square
            for i in 0..9 {
                // if space is open, add TreeNode to vector
                if self.board.state[(next_board*9+i) as usize] == 0 {
                    // calculate new team name
                    let team_name: String;
                    if last_move.team == 1 {
                        team_name = "enemy".parse().unwrap();
                    }else{
                        team_name = TEAM_NAME.parse().unwrap();
                    }
                    // create new move and add to board
                    let new_moove = Moove::new(
                        team_name,
                        *next_board,
                        i); // board + new node
                    let new_node = TreeNode::new(self.board.place_move(new_moove));
                    // push new child
                    all_children.push(new_node);
                }
            }
        }

        return all_children;
    }
}