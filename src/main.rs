extern crate battle_ship;
extern crate collections;

use battle_ship::board;

fn main() {

    let mut board = board::Board::new(10, 10, box vec![board::DefaultShip::new(

        board::Point::new(1, 2),
        board::Point::new(1, 3)
    )]);
    println!("{}", board);

    println!("{}", board.fire(board::Point::new(1,2)));

    println!("{}", board);

    println!("all sunk? {}", board.all_sunk());
    println!("{}", board.fire(board::Point::new(2,2)));

    println!("{}", board);

    println!("all sunk? {}", board.all_sunk());
    println!("{}", board.fire(board::Point::new(1,3)));

    println!("{}", board);

    println!("all sunk? {}", board.all_sunk());
}
