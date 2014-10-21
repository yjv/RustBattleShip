#[deriving(Show)]
pub struct Grid {

    pub width: uint,
    pub height: uint,
    pub points: Box<Vec<Vec<uint>>>
}

impl Grid {

    pub fn new(width: uint, height: uint) -> Grid {

        Grid {
            width: width,
            height: height,
            points: box range(0, width).map(|_| range(0, height).map(|_| 0).collect()).collect()
        }
    }

    pub fn set(&mut self, point: Point, value: uint) {

        *self.points.get_mut(point.y as uint).get_mut(point.x as uint) = value;
    }
}

#[deriving(Show)]
pub struct Board<T: Ship> {

    pub grid: Grid,
    ships: Box<Vec<T>>
}

impl<T: Ship> Board<T> {

    pub fn new(width: uint, height: uint, ships: Box<Vec<T>>) -> Board<T> {

        Board {
           grid: Grid::new(width, height),
           ships: ships
        }
    }

    pub fn all_sunk(&self) -> bool {

        !self.ships.iter().filter(|ship| !ship.is_sunk()).next().is_some()
    }

    pub fn fire(&mut self, point: Point) -> FireResult {

        for ship in self.ships.iter_mut() {

            if ship.contains(point) {

                self.grid.set(point, 1);

                if ship.hit().hit().is_sunk() {

                    return Sink
                }

                return Hit
            }
        }

        Miss
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

#[deriving(Show)]
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

    Hit,
    Miss,
    Sink
}