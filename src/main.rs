use std::{fs, num, time};
use std::thread::sleep;


fn main() {
    let mut board = initialize();
    //print_board(board);
}

fn initialize() -> [char;81]{

    let mut board:[char;81] = [' '; 81];

    println!("Waiting for first four moves...");
    let mut first_four_moves = file_to_string("first_four_moves", true);
    println!("Found first_four_moves.txt: \n {}", first_four_moves);

    println!("Placing first moves on board...");
    if first_four_moves.chars().last().unwrap()!='\n'{
        first_four_moves.push('\n');//appends new line if not already there
    }
    let mut line= String::new();
    for char in first_four_moves.chars() {
        if char == '\n' {
            place_move(line.clone(), &mut board);//because of this clone, this can likely be done async
            line = "".parse().unwrap();
        }else {
            line.push(char);
        }
    }
    return board;
}

fn place_move(move_string: String, board: &mut [char;81]) { //TODO use regex to check format of move_string
    //println!("placing move: {}", move_string);
    //there must be a better way to do this and not have these be mutable
    let mut team: char = '!';
    let mut big_board:u8 = 10;
    let mut small_board:u8 = 10;

    for c in move_string.to_ascii_uppercase().chars(){
        if (c == 'X' || c == 'O') && team == '!' { //only runs if team has yet to be written... this is why it is bad
            team = c;
        }else if c.is_numeric() && c != '9' { //9 is not a valid square
            if big_board == 10 {
                big_board = c.to_digit(10).unwrap() as u8;
            }else if small_board == 10 {
                small_board = c.to_digit(10).unwrap() as u8;
                break;
            }
        }
    }
    println!("Move:{} ({}, {})", team, big_board, small_board);
    board[((big_board*9) + small_board) as usize] = team;
    print_board(*board);

}

fn file_to_string(name: &str, wait_for_file: bool) -> String {

    let file_string_result = fs::read_to_string(name);
    let file_string = match file_string_result {
        Ok(string) => string,
        Err(error) => {
            if !wait_for_file {
                panic!("File Not Found: {} \n Error: {}", name, error)
            }else{
                sleep(time::Duration::from_millis(10));
                return file_to_string(name, wait_for_file);
            }
        }
    };
    return file_string;
}

fn print_board(board: [char;81]){

    println!("-------------------------------------");
    for row in 0..3 {
        for subrow in 0..3 {
            for col in 0..3 {
                for subcol in 0..3 {
                    let rep: String = match board[(row * 27) + (subrow * 3) + (col * 9) + subcol]{
                        ' '=>".".to_string(),
                        'X'=>"❌".to_string(),
                        'O'=>"〇".to_string(),
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