use crate::point::Point2d;
use crate::traits::*;

#[derive(Debug, Default)]
pub struct Wall {
    pub position: Point2d<u16>,
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            position: Point2d::new(x, y),
        }
    }
}

impl Position<u16> for Wall {
    fn position(&self) -> Point2d<u16> {
        Point2d::new(self.position.x, self.position.y)
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }
}
