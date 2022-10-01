use std::fs::read_to_string;
use std::thread::sleep;
use std::time;

pub fn file_to_string(name: &str, wait_for_file: bool) -> String {
    loop{
        let file_string_result = read_to_string(name);
        match file_string_result {
            Ok(string) => return string,
            Err(error) => {
                if !wait_for_file {
                    panic!("File Not Found: {} \n Error: {}", name, error)
                }
            }
        };
        sleep(time::Duration::from_millis(10));
    }
}

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