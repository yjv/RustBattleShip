extern crate battle_ship;
extern crate collections;

use battle_ship::board;
use battle_ship::executor;

fn main() {

    let mut executor = executor::Executor::new(board::Board::new(10, 10, vec![board::DefaultShip::new(

        board::Point::new(1, 2),
        board::Point::new(1, 3)
    )]), executor::CliInput::new(), executor::CliOutput::new());

    executor.execute(10);
}
