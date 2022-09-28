use std::fs;


fn main() {
    let mut board = initialize();
    print_board(board);
}

fn initialize() -> [[char;9]; 9]{

    let mut board:[[char;9]; 9] = [[' '; 9]; 9]; //9x9 array with all characters as SPACE

    println!("Waiting for first four moves...");
    let mut first_four_moves = file_to_string("src/first_four_moves.txt");
    println!("Found first_four_moves.txt: \n {}", first_four_moves);

    println!("Placing first moves on board...");
    first_four_moves.push('\n');
    let mut line= String::new();
    for char in first_four_moves.chars() {
        if char == '\n' {
            //println!("Move: {}", line);
            place_move(line.clone(), &mut board);//because of this clone, this can likely be done async
            line = "".parse().unwrap();
        }else {
            line.push(char);
        }
    }
    return board;
}

fn place_move(move_string: String, board: &mut [[char; 9]; 9]) { //TODO use regex to check format of move_string
    //println!("placing move: {}", move_string);
    //there must be a better way to do this and not have these be mutable
    let mut team: char = '!';
    let mut big_board: u8 = 10;
    let mut small_board: u8 = 10;

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
    board[big_board as usize][small_board as usize] = team; //hate these casts, unsure of better way

}

fn file_to_string(name: &str) -> String {

    let file_string_result = fs::read_to_string(name);
    let file_string = match file_string_result {
        Ok(string) => string,
        Err(error) => panic!("Problem opening the file: {} /n Error: {}", name, error) //TODO: wait and try again to find file
    };
    return file_string;
}

fn print_board(board: [[char;9]; 9]) {

    println!("Board:");
    for row in board {
        println!("-------------------------------------");
        for space in row {
            let rep: char = match space{
                ' '=>' ',
                'X'=>'❌',
                'O'=>'〇',
                _=>'😱', //this symbol is bad
            };
            print!("| {} ", rep);
        }
        println!("|");
    }
    println!("-------------------------------------");

}
