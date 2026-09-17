use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};


#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl From<&(f32, f32)> for Point2D {
    #[inline]
    fn from(&(x, y): &(f32, f32)) -> Self {
        Point2D { x, y }
    }
}

impl From<&[f32; 2]> for Point2D {
    #[inline]
    fn from(&[x, y]: &[f32; 2]) -> Self {
        Point2D { x, y }
    }
}

impl From<&Point2D> for (f32, f32) {
    #[inline]
    fn from(value: &Point2D) -> Self {
        (value.x, value.y)
    }
}

impl From<&Point2D> for [f32; 2] {
    #[inline]
    fn from(value: &Point2D) -> Self {
        [value.x, value.y]
    }
}

impl Add for &Point2D {
    type Output = Point2D;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.x + rhs.y,
        }
    }
}

impl Sub for &Point2D {
    type Output = Point2D;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.x - rhs.y,
        }
    }
}

impl Mul for &Point2D {
    type Output = Point2D;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x * rhs.x,
            y: self.x * rhs.y,
        }
    }
}

impl Add<f32> for &Point2D {
    type Output = Point2D;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Point2D {
            x: self.x + rhs,
            y: self.x + rhs,
        }
    }
}

impl Sub<f32> for &Point2D {
    type Output = Point2D;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Point2D {
            x: self.x - rhs,
            y: self.x - rhs,
        }
    }
}

impl Mul<f32> for &Point2D {
    type Output = Point2D;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Point2D {
            x: self.x * rhs,
            y: self.x * rhs,
        }
    }
}

impl AddAssign<&Point2D> for Point2D {
    #[inline]
    fn add_assign(&mut self, rhs: &Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<&Point2D> for Point2D {
    #[inline]
    fn sub_assign(&mut self, rhs: &Point2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<&Point2D> for Point2D {
    #[inline]
    fn mul_assign(&mut self, rhs: &Point2D) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl AddAssign<f32> for Point2D {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign<f32> for Point2D {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign<f32> for Point2D {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
