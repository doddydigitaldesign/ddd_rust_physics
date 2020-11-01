#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn get_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}
