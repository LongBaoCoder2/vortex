use crate::point::Point2d;
use num::{traits::NumAssign, NumCast};
use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    rngs::ThreadRng,
    Rng,
};
use std::ops::Range;

pub trait Position<T>
where
    T: NumAssign + Copy,
{
    fn position(&self) -> Point2d<T>;
    fn set_position(&mut self, point: Point2d<T>);

    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<T>, y_range: Range<T>)
    where
        T: PartialOrd + SampleUniform,
        Standard: Distribution<T>,
    {
        let position = Point2d::new(rng.gen_range(x_range), rng.gen_range(y_range));
        self.set_position(position);
    }
}

pub trait Movable<T>: Position<T>
where
    T: NumAssign + Copy + NumCast,
{
    const MAX_SPEED: f64 = 3.0;

    const ACCELEBRATE: f64 = 0.2;

    fn accelerate(&mut self);

    fn decelerate(&mut self);

    fn move_forward(&mut self);

    fn forward_position(&self) -> Point2d<T>;

    fn turn_left(&mut self) {}

    fn turn_right(&mut self) {}
}
