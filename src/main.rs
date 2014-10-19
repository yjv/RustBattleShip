extern crate battle_ship;

use battle_ship::battle_ship::board;
use std::io;

fn main() {

    println!("{}", board::Board::new(10, 10))
}
