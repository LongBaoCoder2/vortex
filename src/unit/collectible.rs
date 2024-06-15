use crate::{point::Point2d, traits::*};

#[derive(Debug, Default)]
pub struct Collectible {
    pub position: Point2d<u16>,
}

impl Position<u16> for Collectible {
    fn position(&self) -> Point2d<u16> {
        Point2d::new(self.position.x, self.position.y)
    }

    fn set_position(&mut self, position: Point2d<u16>) {
        self.position = position;
    }
}
