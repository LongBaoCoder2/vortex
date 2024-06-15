use num::traits::NumAssign;

#[derive(Debug, Default)]
pub struct Point2d<T>
where
    T: Copy + NumAssign,
{
    pub x: T,
    pub y: T,
}

impl<T: Copy + NumAssign> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy + NumAssign> PartialEq for Point2d<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
