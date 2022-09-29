use helpers::file_to_string;
use crate::{helpers};

pub struct Moove {
    pub team: String,
    pub big_board: u8,
    pub small_board: u8,
} //like a cow
impl Moove {
    pub(crate) fn parse_from_string(move_string: String) -> Moove {

        let v: Vec<&str> = move_string.split(' ').collect();

        return Moove {
            team: v[0].parse().unwrap(),
            big_board: v[1].chars().nth(0).expect("REASON").to_digit(10).unwrap() as u8,
            small_board: v[2].chars().nth(0).expect("REASON").to_digit(10).unwrap() as u8
        };
    }
}


pub struct Board {
    state: [char;81]
}

impl Board {

    fn new() -> Board {
        Board{state: [' '; 81]}
    }

    pub fn initialize() -> Board {

        let mut b = Board::new();

        println!("Waiting for first four moves...");
        let mut first_four_moves = file_to_string("first_four_moves", true);
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
        self.state[Board::get_index(mv.big_board, mv.small_board)] = mv.team.chars().nth(0).unwrap();
        //self.print();
    }

    pub fn print(&self){
        println!("-------------------------------------");
        for row in 0..3 {
            for subrow in 0..3 {
                for col in 0..3 {
                    for subcol in 0..3 {
                        let rep: String = match self.state[(row * 27) + (subrow * 3) + (col * 9) + subcol]{
                            ' '=>"..".to_string(),
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
}