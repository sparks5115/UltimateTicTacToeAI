use std::fs::{File, read_to_string};
use std::io::Write;
use std::thread::sleep;
use std::time;
use crate::structs::Moove;

/// # Input:
/// An array of 9 that denotes a singular, classic tic tac toe game where 1 is the player, -1 is the opponent, and 0 is an empty square
/// # Output:
/// 0 for draw/not won, 1 for player wins, -1 for opponent wins
pub fn is_board_won(board_state: &[i8]) -> i8{
    for row in 0..2{
        let mut row_start_idx = row*3;
        let mut row_total: i8 = 0;
        for i in 0..2{
            row_total += board_state[row_start_idx + i];
        }
        if row_total == 3 {return 1;}
        if row_total == -3 {return -1;}
    }
    for col in 0..2{
        let mut col_total: i8 = 0;
        for i in 0..2{
            col_total += board_state[col + (3*i)];
        }
        if col_total == 3 {return 1;}
        if col_total == -3 {return -1;}
    }
    let diag1_total = board_state[0] + board_state[4] + board_state[8];
    if diag1_total == 3 {return 1;}
    if diag1_total == -3 {return -1;}

    let diag2_total = board_state[2] + board_state[4] + board_state[6];
    if diag2_total == 3 {return 1;}
    if diag2_total == -3 {return -1;}

    return 0; //no winner
}

///writes to the move_file
pub fn write_to_move_file(moove:Moove){
    let mut f = File::create("move_file");
    f.expect("Failed to open move_file")
        .write_all(format!("{} {} {}", moove.team, moove.big_board, moove.small_board).as_ref())
        .expect("Failed to write to move_file");
}