use sdl2::rect::Point;

#[derive(PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub y: f64,
    pub x: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn from_point(point: Point) -> Vector2 {
        Vector2::new(point.x as f64, point.y as f64)
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        *self = *self / self.norm();
    }

    pub fn set_mag(&mut self, mag: f64) {
        *self = *self / self.norm() * mag;
    }


    pub fn limit(&mut self, max: f64) {
        if self.norm() > max {
            self.set_mag(max);
        }
    }

    pub fn low(&mut self, min: f64) {
        if self.norm() < min {
            self.set_mag(min);
        }
    }
}

use std::ops::{Add, AddAssign, Sub, Mul, MulAssign, Div};

impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, scalar: f64) -> Self::Output {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;
    fn div(self, scalar: f64) -> Self::Output {
        Vector2::new(self.x / scalar, self.y / scalar)
    }
}

use std::fmt;

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
