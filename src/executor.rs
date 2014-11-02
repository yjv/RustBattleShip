use board;
use std::io;

pub trait Input {

    fn get_point(&self) -> board::Point;
}

pub struct CliInput;

impl CliInput {

    pub fn new() -> CliInput {

        CliInput
    }
}

impl Input for CliInput {

    fn get_point(&self) -> board::Point {

        println!("Guess row: ");
        let y = from_str::<int>(io::stdin().read_line().ok().expect("Failed to read line").as_slice().trim()).unwrap() - 1;
        println!("Guess column: ");
        let x = from_str::<int>(io::stdin().read_line().ok().expect("Failed to read line").as_slice().trim()).unwrap() - 1;

        board::Point {

            x: x,
            y: y
        }
    }
}

pub trait Output<T: board::Ship> {

    fn welcome(&self, board: &board::Board<T>);
    fn grid(&self, grid: &board::Grid);
    fn hit(&self);
    fn sink(&self);
    fn miss(&self);
    fn lost(&self);
    fn won(&self);
    fn error(&self, message: String);
    fn turn(&self, turn: uint, max_turn: uint);
}

pub struct CliOutput;

impl CliOutput {

    pub fn new() -> CliOutput {

        CliOutput
    }
}

impl <T: board::Ship> Output<T> for CliOutput {

    fn welcome(&self, board: &board::Board<T>) {

        println!(
            "Welcome to mini battleship! You have ten turns to egusss where my battleship is on this {} by {} square by by guessing a row and a column",
            board.grid.width,
            board.grid.height
        );
    }

    fn grid(&self, grid: &board::Grid) {

        println!("{}", grid);
    }

    fn hit(&self) {

        println!("Hit!");
    }

    fn sink(&self) {

        println!("Sunk!");
    }

    fn miss(&self) {

        println!("You missed my battleship");
    }

    fn lost(&self) {

        println!("Game over. So sorry. You lost.");
    }

    fn won(&self) {

        println!("Congratulations! You sunk all my battleships!");
    }

    fn error(&self, message: String) {

        println!("Oops, {}", message);
    }

    fn turn(&self, turn: uint, max_turns: uint) {

        println!("Turn {} of {}", turn, max_turns);
    }
}

pub struct Executor<T: Input, U: Output<V>, V: board::Ship> {

    input: T,
    output: U,
    board: board::Board<V>
}

impl <T: Input, U: Output<V>, V: board::Ship> Executor<T, U, V> {

    pub fn new(board: board::Board<V>, input: T, output: U) -> Executor<T, U, V> {

        Executor {

            input: input,
            output: output,
            board: board
        }
    }

    pub fn execute(&mut self, max_turns: uint) {

        self.output.welcome(&self.board);

        let mut turns = 1;

        loop {

            self.output.turn(turns, max_turns);
            self.output.grid(&self.board.grid);

            match self.board.fire(self.input.get_point()) {

                Ok(board::Hit(board::NotSunk)) => self.output.hit(),
                Ok(board::Hit(board::Sunk)) => self.output.sink(),
                Ok(board::Miss) => self.output.miss(),
                Err(board::InvalidSelectionError) => self.output.error(String::from_str("that's not even in the ocean")),
                Err(board::AlreadyGuessedError) => self.output.error(String::from_str("you guessed that one already"))
            };

            if self.board.all_sunk() {

                self.output.won();
                break;
            }

            if turns == max_turns {

                self.output.lost();
                break;
            }

            turns += 1;
        }
    }
}