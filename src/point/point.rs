use std::ops::{Add, Sub};

use num::traits::NumAssign;

#[derive(Debug, Default, Clone)]
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

impl<T: Copy + NumAssign + num::Float> Point2d<T> {
    pub fn normalize(mut self) -> Self {
        let magnitude = self.x * self.x + self.y * self.y;
        self.x /= magnitude;
        self.y /= magnitude;
        self
    }
}

impl<T: Copy + NumAssign> PartialEq for Point2d<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<'a, T> Add<&'_ Point2d<T>> for Point2d<T>
where
    T: Copy + NumAssign,
{
    type Output = Point2d<T>;

    fn add(self, other: &'_ Point2d<T>) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, T> Add for &'_ Point2d<T>
where
    T: Copy + NumAssign,
{
    type Output = Point2d<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Copy + NumAssign> Sub for Point2d<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, T> Sub<&'_ Point2d<T>> for Point2d<T>
where
    T: Copy + NumAssign,
{
    type Output = Self;

    fn sub(self, other: &'_ Point2d<T>) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, T> Sub for &'_ Point2d<T>
where
    T: Copy + NumAssign,
{
    type Output = Point2d<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
