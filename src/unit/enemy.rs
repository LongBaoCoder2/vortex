use crate::{point::Point2d, traits::*};

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Enemy {
    pub speed: f64,
    pub position: Point2d<f64>,
}

impl Enemy {
    pub fn with_speed(speed: f64) -> Self {
        Self {
            speed,
            position: Point2d::default(),
        }
    }
}

impl Position<f64> for Enemy {
    fn position(&self) -> Point2d<f64> {
        Point2d::new(self.position.x, self.position.y)
    }

    fn set_position(&mut self, position: Point2d<f64>) {
        self.position = position;
    }
}
