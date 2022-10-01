use helpers::file_to_string;
use crate::{helpers};
use crate::helpers::is_board_won;
use super::TEAM_NAME;

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

pub struct Moove {//like a cow
    pub team: String,
    pub big_board: u8,
    pub small_board: u8,
}
impl Moove {
    pub fn parse_from_string(move_string: String) -> Moove {

        let v: Vec<&str> = move_string.trim().split(' ').collect(); //this only works if it is formatted correctly

        return Moove {
            team: v[0].parse().unwrap(),
            big_board: v[1].chars().nth(0).expect("Invalid move_string").to_digit(10).unwrap() as u8,
            small_board: v[2].chars().nth(0).expect("Invalid move_string").to_digit(10).unwrap() as u8,
        };
    }
}


pub struct Board {
    state: [i8;81]
}
impl Board {

    fn new() -> Board {
        Board{state: [0; 81]}
    }

    pub fn initialize() -> Board {

        let mut b = Board::new();

        println!("Waiting for first four moves...");
        let mut first_four_moves = file_to_string(r"first_four_moves", true);
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

    pub fn get_index(big_board: u8, small_board: u8) -> usize{
        ((big_board*9) + small_board) as usize
    }

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

    pub fn get_heuristic_value(&self) -> i32{
        print!("Getting heuristic...");
        let big_board_state = self.get_big_board_state();
        let mut h_val = 0;
        let temp = self.is_winning_or_losing() as i32;
        if temp != 0 {
            return temp * i32::MAX; //if the state is winning or losing, there is no need to continue
        }

        //todo all these
        // h_val += HEURISTIC.board_win_loss * Board.net_boards_won();
        // h_val += HEURISTIC.two_boards_in_row * Board.net_two_boards_in_row();
        // h_val += HEURISTIC.block_opponent_board * Board.net_blocked_boards();
        // h_val += HEURISTIC.useless_board_win * Board.net_useless_boards();
        // h_val += HEURISTIC.two_in_row * Board.net_two_in_row();
        // h_val += HEURISTIC.block_opponent * Board.net_blocked();
        // h_val += HEURISTIC.useless_move * Board.net_useless();
        return h_val;
    }

    pub fn is_winning_or_losing(&self) -> i8{
        is_board_won(&self.get_big_board_state()[..])
    }

    pub fn get_big_board_state(&self) -> [i8; 9]{
        let mut big_board: [i8; 9] = [0 as i8; 9];
        for i in 0..8 {
            big_board[i] = self.board_is_won(i as u8);
        }
        return big_board;
    }

    pub fn board_is_won(&self, board_number:u8) -> i8{
        return helpers::is_board_won(self.get_small_board_state(board_number));
    }

    pub fn get_small_board_state(&self, board_number:u8) -> &[i8] {
        &self.state[(board_number*9) as usize .. (board_number*9 + 9) as usize]
    }

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

pub struct Heuristic {
    total_win_loss: i32, //game win or loss
    board_win_loss: i32, //win or loss on small board
    two_boards_in_row: i32, //win two boards in a row
    block_opponent_board: i32, //block opponent's two in a row
    useless_board_win: i32, //board that is blocked
    two_in_row: i32, //two in a row on small board
    block_opponent: i32, //block on small board
    useless_move: i32, //blocked move on small board
    allow_opponent_wildcard: i32 //send opponent to won/full board, allowing them to move anywhere
}

pub struct TreeNode {
    board: Board,
    heuristic_value: i32,
    children: Vec<TreeNode>
}