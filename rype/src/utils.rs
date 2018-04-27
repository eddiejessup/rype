extern crate num;

use std::ops::{Add, Sub, Mul, Div, AddAssign};
use self::num::ToPrimitive;

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point {x: x, y: y}
    }

    pub fn sub(&self, other: &Point<T>) -> Point<T>
    where
        T: Sub<Output=T> + Copy
    {
        Point::new(other.x - self.x, other.y - self.y)
    }

    pub fn add(&self, other: &Point<T>) -> Point<T>
    where
        T: Add<Output=T> + Copy
    {
        Point::new(other.x + self.x, other.y + self.y)
    }

    pub fn mul(&self, other: &Point<T>) -> Point<T>
    where
        T: Mul<Output=T> + Copy
    {
        Point {x: other.x * self.x, y: other.y * self.y}
    }

    pub fn add_inplace(&mut self, other: &Point<T>)
    where
        T: AddAssign + Copy
    {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn scale(&self, scale: T) -> Point<T>
    where
        T: Mul<Output=T> + Copy
    {
        Point {x: self.x * scale, y: self.y * scale}
    }

    pub fn scale_down(&self, scale: T) -> Point<T>
    where
        T: Div<Output=T> + Copy
    {
        Point {x: self.x / scale, y: self.y / scale}
    }

    pub fn mag_sq(&self) -> T
    where
        T: Mul<Output=T> + Add<Output=T> + Copy
    {
        self.x * self.x + self.y * self.y
    }

    pub fn mag(&self) -> f64
    where
        T: Mul<Output=T> + Add<Output=T> + Copy + ToPrimitive
    {
        self.mag_sq().to_f64().unwrap().sqrt()
    }
}

impl Point<f64> {
    pub fn as_unit(&self) -> Point<f64>
    {
        self.scale_down(self.mag())
    }
}
