extern crate battle_ship;
extern crate collections;

use battle_ship::board;
use std::io;

fn main() {

    println!("{}", board::Board::new(10, 10, Vec<board::Ship>::new()))
}
