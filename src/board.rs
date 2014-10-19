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
}

pub trait Ship {

}

#[deriving(Show)]
pub struct DefaultShip;

impl Ship for DefaultShip {

}