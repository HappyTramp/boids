use sdl2::rect::Point;

#[derive(PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub y: f32,
    pub x: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn from_point(point: Point) -> Vector2 {
        Vector2::new(point.x as f32, point.y as f32)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        *self = *self / self.norm();
    }
}

use std::ops::{Add, Mul, MulAssign, Div};

impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, scalar: f32) -> Self::Output {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, scalar: f32) -> Self::Output {
        Vector2::new(self.x / scalar, self.y / scalar)
    }
}
