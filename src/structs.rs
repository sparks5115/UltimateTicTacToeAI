use std::fs::read_to_string;
use crate::{helpers};
use crate::helpers::{is_board_won, is_two_in_row_us};
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
    allow_opponent_wildcard: -75 //todo test this maybe?
};

///holds a move made by either team
pub struct Moove {//like a cow
    pub team: String,
    pub big_board: u8,
    pub small_board: u8,
}
impl Moove { //implementation? Inside here are functions for the struct
    ///parses a string (such as the one in move_file) into a Moove struct
    /// # Examples:
    /// m: Moove = Moove::parse_from_string(read_to_string("move_file"));
    pub fn parse_from_string(move_string: String) -> Moove {

        let v: Vec<&str> = move_string.trim().split(' ').collect(); //this only works if it is formatted correctly

        return Moove {
            team: v[0].parse().unwrap(),
            big_board: v[1].chars().nth(0).expect("Invalid move_string").to_digit(10).unwrap() as u8,
            small_board: v[2].chars().nth(0).expect("Invalid move_string").to_digit(10).unwrap() as u8,
        };
    }
}

///state of the board is stored in an array as follows:
/// big_board number 0 takes up spaces 0-8
/// big_board number 1 takes up spaces 9-15
pub struct Board {
    state: [i8;81]
}
impl Board {

    fn new() -> Board {
        Board{state: [0; 81]}
    }

    pub fn initialize() -> Board {

        let mut b = Board::new();

        println!("Waiting for first four moves..."); // this line + next line + 5 ahead for checking if game won
        let mut ffm_result = read_to_string(r"first_four_moves");
        while ffm_result.is_err() { //block until it finds the file
            ffm_result = read_to_string(r"first_four_moves");
        }
        let mut first_four_moves: String = ffm_result.expect("wohoo"); // becuase you wait until there is not an error teachnically this isn't needed, but it'll freak anyways

        println!("Found first_four_moves.txt: \n {}", first_four_moves);

        println!("Placing first moves on Board...");
        if first_four_moves.chars().last().unwrap()!='\n'{
            first_four_moves.push('\n');//appends new line if not already there
        }
        let mut line= String::new();
        for char in first_four_moves.chars() {
            if char == '\n' {
                b.place_move(Moove::parse_from_string(line.clone()));//because of this clone, this can likely be done async
                line = "".parse().unwrap();
            }else {
                line.push(char);
            }
        }
        return b;
    }

    ///Converts from notation of big_board, small_board into the index in Board's state
    pub fn get_index(big_board: u8, small_board: u8) -> usize{
        ((big_board*9) + small_board) as usize
    }

    ///called on a board, places a Moove onto its own state
    pub fn place_move(&mut self, mv:Moove){
        println!("Move:{} ({}, {})", mv.team, mv.big_board, mv.small_board);
        let symbol:i8;
        if mv.team.to_ascii_uppercase() == TEAM_NAME.to_ascii_uppercase() {
            symbol = 1;
        }else { symbol = -1 }
        self.state[Board::get_index(mv.big_board, mv.small_board)] =symbol;
        //self.print();
        //TODO this should call a function to write this move to the file
    }

    ///gets the heuristic of the board that it is called on
    pub fn get_heuristic_value(&self) -> i32{
        print!("Getting heuristic...");
        let big_board_state = self.get_big_board_state();
        let mut h_val = 0;
        let temp = self.is_winning_or_losing(Some(big_board_state)) as i32;
        if temp != 0 {
            return temp * i32::MAX; //if the state is winning or losing, there is no need to continue
        }

        //todo all these
        h_val += HEURISTIC.board_win_loss * (self.net_boards_won(Some(big_board_state)) as i32);
        h_val += HEURISTIC.two_boards_in_row * (self.net_two_boards_in_row(Some(big_board_state)) as i32);
        // h_val += HEURISTIC.block_opponent_board * self.net_blocked_boards();
        // h_val += HEURISTIC.useless_board_win * self.net_useless_boards();
        // h_val += HEURISTIC.two_in_row * self.net_two_in_row();
        // h_val += HEURISTIC.block_opponent * self.net_blocked();
        // h_val += HEURISTIC.useless_move * self.net_useless();
        return h_val;
    }

    ///checks if the game is over, and one team has won (checks if it's a terminal node)
    /// # Returns:
    /// -1 if opponent has won
    /// 1 if we have won
    /// 0 if not won
    pub fn is_winning_or_losing(&self, big_board_state: Option<[i8; 9]>) -> i8{
        let bbs = match big_board_state {
            None => {self.get_big_board_state()},
            Some(a) => {a},
        };
        is_board_won(&bbs[..])
    }

    ///calculates boards won by us, minus boards won by opponent
    pub fn net_boards_won(&self, big_board_state: Option<[i8; 9]>) -> i8{
        let bbs = match big_board_state {
            None => {self.get_big_board_state()},
            Some(a) => {a},
        };
        bbs.iter().sum()
    }

    /// calculates how many sets of 2 big boards in a row there are
    /// TODO: Why is it when you directly return num_two_boards_in_row in the first line does it error
    pub fn net_two_boards_in_row(&self, big_board_state: Option<[i8; 9]>) -> i8 {
        let num_two_boards_in_row = is_two_in_row_us(big_board_state: &[i8]);

        return num_two_boards_in_row;
    }

    ///checks the entire board.state and returns an array representing the board as a singular tic tac toe board.
    ///  (if you ignore that the individual games are games and treat them as their results, it treats the big board as a simple tic tac toe game)
    pub fn get_big_board_state(&self) -> [i8; 9]{
        let mut big_board: [i8; 9] = [0 as i8; 9];
        for i in 0..8 {
            big_board[i] = self.board_is_won(i as u8);
        }
        return big_board;
    }

    ///wraps is_board_won for convenience such that it takes a board number as an argument instead of an array representing a board
    /// # Returns:
    /// -1 if opponent has won
    /// 1 if we have won
    /// 0 if not won
    pub fn board_is_won(&self, board_number:u8) -> i8{
        return is_board_won(self.get_small_board_state(board_number));
    }

    ///returns an array of length 9 representing a single small board in the game given a board number
    pub fn get_small_board_state(&self, board_number:u8) -> &[i8] {
        &self.state[(board_number*9) as usize .. (board_number*9 + 9) as usize]
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

//TODO could likely change all of the heuristic stuff to i16 for space
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
    allow_opponent_wildcard: i32 //send opponent to won/full board, allowing them to move anywhere
}

///wraps a board and all the information needed to run the minimax algorithm
pub struct TreeNode {
    pub board: Board,
    pub last_move: Moove,
    pub heuristic_value: i32,
    pub children: Vec<TreeNode>
}

impl TreeNode{
    ///builds all of this node's children (a collection of tree nodes denoting the next legal moves that could be made)
    pub fn find_all_children(&self){
        //TODO
    }
}