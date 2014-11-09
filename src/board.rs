use std::collections::HashMap;

#[deriving(Show)]
pub struct Grid {

    pub width: uint,
    pub height: uint,
    pub points: HashMap<Point, FireResult>
}

impl Grid {

    pub fn new(width: uint, height: uint) -> Grid {

        Grid {
            width: width,
            height: height,
            points: HashMap::<Point, FireResult>::new()
        }
    }

    pub fn update_point(&mut self, point: Point, result: FireResult) {

        self.points.insert(point, result);
    }

    pub fn is_already_set(&self, point: Point) -> bool {

        self.points.contains_key(&point)
    }

    pub fn get_result(&self, point: Point) -> Option<&FireResult> {

        self.points.get(&point)
    }

    pub fn contains(&self, point: Point) -> bool {

        if point.x < 0 || point.y < 0 {

            return false;
        }

        point.x as uint <= self.width && point.y as uint <= self.height
    }
}

#[deriving(Show)]
pub struct Board<T: Ship> {

    pub grid: Grid,
    ships: Vec<T>
}

impl<T: Ship> Board<T> {

    pub fn new(width: uint, height: uint, ships: Vec<T>) -> Board<T> {

        Board {
           grid: Grid::new(width, height),
           ships: ships
        }
    }

    pub fn all_sunk(&self) -> bool {

        !self.ships.iter().filter(|ship| !ship.is_sunk()).next().is_some()
    }

    pub fn fire(&mut self, point: Point) -> Result<FireResult, FireError> {

        if !self.grid.contains(point) {

            return Err(InvalidSelectionError);
        }

        if self.grid.is_already_set(point) {

            return Err(AlreadyGuessedError);
        }

        for ship in self.ships.iter_mut() {

            if ship.contains(point) {

                ship.hit();
                let is_sunk = if ship.is_sunk() { Sunk } else { NotSunk };
                self.grid.update_point(point, Hit(is_sunk));
                return Ok(Hit(is_sunk));
            }
        }

        self.grid.update_point(point, Miss);
        Ok(Miss)
    }
}

pub trait Ship {

    fn contains(&self, point: Point) -> bool;
    fn hit(&mut self) -> &mut Self;
    fn is_sunk(&self) -> bool;
}

#[deriving(Show)]
pub struct DefaultShip {

    pub point1: Point,
    pub point2: Point,
    hits: int,
    max_hits: int
}

impl DefaultShip {

    pub fn new(point1: Point, point2: Point) -> DefaultShip {

        DefaultShip {

            point1: point1,
            point2: point2,
            hits: 0,
            max_hits: DefaultShip::get_distance(point1, point2) + 1
        }
    }

    fn get_distance(point1: Point, point2: Point) -> int {

        ((point1.x - point2.x) as f32).hypot((point1.y - point2.y) as f32) as int
    }
}

impl Ship for DefaultShip {

    fn contains(&self, point: Point) -> bool {

        DefaultShip::get_distance(self.point1, point) + DefaultShip::get_distance(point, self.point2) + 1 == self.max_hits
    }

    fn hit(&mut self) -> &mut DefaultShip {

        self.hits += 1;
        self
    }

    fn is_sunk(&self) -> bool {

        self.hits >= self.max_hits
    }
}

#[deriving(PartialEq,Eq,Hash,Show)]
pub struct Point {

    pub x: int,
    pub y: int
}

impl Point {

    pub fn new(x: int, y: int) -> Point {

        Point {

            x: x,
            y: y
        }
    }
}

#[deriving(Show)]
pub enum FireResult {

    Hit(IsSunk),
    Miss
}

#[deriving(Show)]
pub enum FireError {

    InvalidSelectionError,
    AlreadyGuessedError
}

#[deriving(Show)]
pub enum IsSunk {

    Sunk,
    NotSunk
}