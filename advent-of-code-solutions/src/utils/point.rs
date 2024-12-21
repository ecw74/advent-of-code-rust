use num::traits::{Num, Signed};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Signed + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    /// Rotates the direction counter-clockwise.
    pub fn rotate_ccw(&self) -> Point<T> {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    /// Rotates the direction clockwise.
    pub fn rotate_cw(&self) -> Point<T> {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn distance(&self, other: Point<T>) -> Point<T> {
        Point::new(self.x - other.x, self.y - other.y)
    }

    pub fn abs(&self) -> Point<T> {
        Point::new(self.x.abs(), self.y.abs())
    }
}

impl<T> Point<T>
where
    T: Signed + Num + Copy,
{
    /// Calculates the Manhattan distance between two points.
    pub fn manhattan(&self, other: Point<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<T> Add for Point<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
