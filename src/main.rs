extern crate battle_ship;
extern crate collections;

use battle_ship::board;
use std::io;

fn main() {

    println!("{}", board::Board::new(10, 10, vec![board::DefaultShip::new(

        board::Point::new(10, 20),
        board::Point::new(10, 21)
    )]))
}
