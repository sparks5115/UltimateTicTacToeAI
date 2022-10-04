use std::fs::{File, read_to_string};
use std::io::Write;
use std::thread::sleep;
use std::time;
use crate::structs::Moove;
use crate::TEAM_NAME;

/// # Input:
/// An array of 9 that denotes a singular, classic tic tac toe game where 1 is the player, -1 is the opponent, and 0 is an empty square
/// # Output:
/// 0 for draw/not won, 1 for player wins, -1 for opponent wins
pub fn is_board_won(board_state: &[i8]) -> i8{
    for row in 0..3{
        let mut row_start_idx = row*3;
        let mut row_total: i8 = 0;
        for i in 0..3{
            row_total += board_state[row_start_idx + i];
        }
        if row_total == 3 {return 1;}
        if row_total == -3 {return -1;}
    }
    for col in 0..3{
        let mut col_total: i8 = 0;
        for i in 0..3{
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

/// function that determines if our team has 2 elements in a row with a third open.
/// Returns: n, with n being the total number of pairs
pub fn is_two_in_row_us(board_state: &[i8]) -> i8{
    let mut total_pairs = 0;
    for row in 0..3{
        let mut row_start_idx = row*3;
        let mut row_total: i8 = 0;
        for i in 0..3{
            row_total += board_state[row_start_idx + i];
        }
        if row_total == 2 {total_pairs+=1;}
        //if row_total == -2 {return -1;}
    }
    for col in 0..3{
        let mut col_total: i8 = 0;
        for i in 0..3{
            col_total += board_state[col + (3*i)];
        }
        if col_total == 2 {total_pairs+=1;}
        //if col_total == -2 {return -1;}
    }
    let diag1_total = board_state[0] + board_state[4] + board_state[8];
    if diag1_total == 2 {total_pairs+=1;}
    //if diag1_total == -2 {return -1;}

    let diag2_total = board_state[2] + board_state[4] + board_state[6];
    if diag2_total == 2 {total_pairs+=1;}
    //if diag2_total == -2 {return -1;}

    return total_pairs;
}

/// function that determines if their team has 2 elements in a row with a third open.
/// Returns: n, with n being the total number of pairs
pub fn is_two_in_row_them(board_state: &[i8]) -> i8{
    let mut total_pairs = 0;
    for row in 0..3{
        let mut row_start_idx = row*3;
        let mut row_total: i8 = 0;
        for i in 0..3{
            row_total += board_state[row_start_idx + i];
        }
        if row_total == -2 {total_pairs+=1;}
        //if row_total == -2 {return -1;}
    }
    for col in 0..3{
        let mut col_total: i8 = 0;
        for i in 0..3{
            col_total += board_state[col + (3*i)];
        }
        if col_total == -2 {total_pairs+=1;}
        //if col_total == -2 {return -1;}
    }
    let diag1_total = board_state[0] + board_state[4] + board_state[8];
    if diag1_total == -2 {total_pairs+=1;}
    //if diag1_total == -2 {return -1;}

    let diag2_total = board_state[2] + board_state[4] + board_state[6];
    if diag2_total == -2 {total_pairs+=1;}
    //if diag2_total == -2 {return -1;}

    return total_pairs;
}

// block opponent note: if sum is 1 and there are no empty spaces, it's us us them
//          if sum is -1 and there are no empty spaces, it's them them us

/// function that counts the net number of rows, cols, and diagonals that are blocked (XXO)
pub fn block_opponent(board_state: &[i8]) -> i8{
    let mut total_blocks = 0;
    for row in 0..3{
        let mut is_something_empty = false;
        let mut row_start_idx = row*3;
        let mut row_total: i8 = 0;
        for i in 0..3{
            if i == 0 { is_something_empty = true; }
            row_total += board_state[row_start_idx + i];
        }
        if (row_total == 1) && (is_something_empty == false) {total_blocks+=1;}
        else if (row_total == -1) && (is_something_empty == false) {total_blocks-=1;}
    }
    for col in 0..3{
        let mut is_something_empty = false;
        let mut col_total: i8 = 0;
        for i in 0..3{
            if i == 0 { is_something_empty = true; }
            col_total += board_state[col + (3*i)];
        }
        if (col_total == 1) && (is_something_empty == false) {total_blocks+=1;}
        else if (col_total == -1) && (is_something_empty == false) {total_blocks-=1;}
    }
    let diag1_total = board_state[0] + board_state[4] + board_state[8];
    if (diag1_total == 1) && (board_state[0] != 0) && (board_state[4] != 0) && (board_state[8] != 0) {total_blocks+=1;}
    else if (diag1_total == -1) && (board_state[0] != 0) && (board_state[4] != 0) && (board_state[8] != 0) {total_blocks-=1;}

    let diag2_total = board_state[2] + board_state[4] + board_state[6];
    if (diag2_total == 1) && (board_state[2] != 0) && (board_state[4] != 0) && (board_state[6] != 0) {total_blocks+=1;}
    else if (diag2_total == -1) && (board_state[2] != 0) && (board_state[4] != 0) && (board_state[6] != 0) {total_blocks-=1;}
    return total_blocks;
}

///writes to the move_file
pub fn write_to_move_file(moove:Moove){
    let mut f = File::create("move_file");
    f.expect("Failed to open move_file")
        .write_all(format!("{} {} {}", TEAM_NAME, moove.big_board, moove.small_board).as_ref())
        .expect("Failed to write to move_file");
}