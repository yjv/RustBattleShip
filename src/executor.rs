use board;

pub trait Input {

    fn get_point(&self) -> board::Point;
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
    fn turn(&self, turn: int, max_turn: int);
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

    pub fn execute(&mut self, max_turns: int) {

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