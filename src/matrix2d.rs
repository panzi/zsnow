use std::ops::Mul;

use crate::point2d::Point2D;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix2d(pub [f32; 4]);

impl Matrix2d {
    pub const UNIT: Self = Self([
        1.0, 0.0,
        0.0, 1.0,
    ]);

    #[inline]
    pub fn from_rotation(rad: f32) -> Self {
        Self([
            rad.cos(), -rad.sin(),
            rad.sin(), rad.cos(),
        ])
    }

    #[inline]
    pub fn from_scale(sx: f32, sy: f32) -> Self {
        Self([
            sx, 0.0,
            0.0, sy,
        ])
    }
}

impl Mul for &Matrix2d {
    type Output = Matrix2d;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = &self.0;
        let rhs = &rhs.0;

        Matrix2d([
            lhs[0] * rhs[0] + lhs[1] * rhs[2],
            lhs[0] * rhs[1] + lhs[1] * rhs[3],

            lhs[2] * rhs[0] + lhs[3] * rhs[2],
            lhs[2] * rhs[1] + lhs[3] * rhs[3],
        ])
    }
}

impl Mul<&Point2D> for &Matrix2d {
    type Output = Point2D;

    #[inline]
    fn mul(self, rhs: &Point2D) -> Self::Output {
        let lhs = &self.0;

        Point2D {
            x: lhs[0] * rhs.x + lhs[1] * rhs.y,
            y: lhs[2] * rhs.x + lhs[3] * rhs.y,
        }
    }
}

impl Mul<&Matrix2d> for &Point2D {
    type Output = Point2D;

    #[inline]
    fn mul(self, rhs: &Matrix2d) -> Self::Output {
        let rhs = &rhs.0;

        Point2D {
            x: self.x * rhs[0] + self.y * rhs[2],
            y: self.x * rhs[1] + self.y * rhs[3],
        }
    }
}

impl Mul<&(f32, f32)> for &Matrix2d {
    type Output = (f32, f32);

    #[inline]
    fn mul(self, rhs: &(f32, f32)) -> Self::Output {
        let lhs = &self.0;

        (
            lhs[0] * rhs.0 + lhs[1] * rhs.1,
            lhs[2] * rhs.0 + lhs[3] * rhs.1,
        )
    }
}

impl Mul<&Matrix2d> for &(f32, f32) {
    type Output = (f32, f32);

    #[inline]
    fn mul(self, rhs: &Matrix2d) -> Self::Output {
        let rhs = &rhs.0;

        (
            self.0 * rhs[0] + self.1 * rhs[2],
            self.0 * rhs[1] + self.1 * rhs[3],
        )
    }
}

impl Mul<&[f32; 2]> for &Matrix2d {
    type Output = [f32; 2];

    #[inline]
    fn mul(self, rhs: &[f32; 2]) -> Self::Output {
        let lhs = &self.0;

        [
            lhs[0] * rhs[0] + lhs[1] * rhs[1],
            lhs[2] * rhs[0] + lhs[3] * rhs[1],
        ]
    }
}

impl Mul<&Matrix2d> for &[f32; 2] {
    type Output = [f32; 2];

    #[inline]
    fn mul(self, rhs: &Matrix2d) -> Self::Output {
        let rhs = &rhs.0;

        [
            self[0] * rhs[0] + self[1] * rhs[2],
            self[0] * rhs[1] + self[1] * rhs[3],
        ]
    }
}
