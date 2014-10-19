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
            points: range(0, width).map(|x| range(0, height).map(|x| 0).collect()).collect()
        }
    }
}

#[deriving(Show)]
pub struct Board {

    pub grid: Grid
}

impl Board {

    pub fn new(width: uint, height: uint) -> Board {

        Board {
           grid: Grid::new(width, height)
        }
    }
}

trait Ship {
}