mod structs;
mod helpers;

use structs::{Moove, Board};
use std::{fs, time};
use std::borrow::Borrow;
use std::thread::sleep;
use crate::helpers::file_to_string;


static  TEAM_NAME:&str = "Temp"; //TODO come up with real team name

pub fn main() {
    let mut board = Board::initialize();
    board.print();
}