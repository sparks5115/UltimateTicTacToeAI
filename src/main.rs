mod structs;
mod helpers;

use structs::Board;


pub const TEAM_NAME:&str = "TEMP"; //TODO come up with real team name

pub fn main() {
    let mut board = Board::initialize();
    board.print();
}