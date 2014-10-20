#[deriving(Show)]
pub struct Grid {

    pub width: uint,
    pub height: uint,
    pub points: Vec<Vec<uint>>
}

impl Grid {

    pub fn new(width: uint, height: uint) -> Grid {

        Grid {
            width: width,
            height: height,
            points: range(0, width).map(|_| range(0, height).map(|_| 0).collect()).collect()
        }
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

        self.ships.len() == 0
    }

    pub fn fire(&mut self, point: Point) -> FireResult<T> {

        Miss
    }
}

pub trait Ship {

    fn contains(&self, point: Point) -> bool;
    fn hit(&mut self) -> bool;
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

    fn hit(&mut self) -> bool {

        self.hits += 1;

        if (self.hits >= self.max_hits) {

            return true
        }

        false
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

pub enum FireResult<T: Ship> {

    Hit,
    Miss,
    Sink(T)
}