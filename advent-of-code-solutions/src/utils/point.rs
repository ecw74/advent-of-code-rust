use num::abs;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    /// Rotates the direction counter-clockwise.
    pub(crate) fn rotate_ccw(&self) -> Point {
        Point {
            x: self.y,
            y: -self.x,
        }
    }

    /// Rotates the direction clockwise.
    pub(crate) fn rotate_cw(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    /// calc manhattan distance between two points
    pub(crate) fn manhattan(&self, other: Point) -> i16 {
        abs(self.x - other.x) + abs(self.y - other.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
