use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};


#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for &Point3D {
    type Output = Point3D;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x + rhs.x,
            y: self.x + rhs.y,
            z: self.x + rhs.z,
        }
    }
}

impl Sub for &Point3D {
    type Output = Point3D;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x - rhs.x,
            y: self.x - rhs.y,
            z: self.x - rhs.z,
        }
    }
}

impl Mul for &Point3D {
    type Output = Point3D;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Point3D {
            x: self.x * rhs.x,
            y: self.x * rhs.y,
            z: self.x * rhs.z,
        }
    }
}

impl Add<f32> for &Point3D {
    type Output = Point3D;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        Point3D {
            x: self.x + rhs,
            y: self.x + rhs,
            z: self.x + rhs,
        }
    }
}

impl Sub<f32> for &Point3D {
    type Output = Point3D;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        Point3D {
            x: self.x - rhs,
            y: self.x - rhs,
            z: self.x - rhs,
        }
    }
}

impl Mul<f32> for &Point3D {
    type Output = Point3D;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Point3D {
            x: self.x * rhs,
            y: self.x * rhs,
            z: self.x * rhs,
        }
    }
}

impl AddAssign<&Point3D> for Point3D {
    #[inline]
    fn add_assign(&mut self, rhs: &Point3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<&Point3D> for Point3D {
    #[inline]
    fn sub_assign(&mut self, rhs: &Point3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<&Point3D> for Point3D {
    #[inline]
    fn mul_assign(&mut self, rhs: &Point3D) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl AddAssign<f32> for Point3D {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl SubAssign<f32> for Point3D {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl MulAssign<f32> for Point3D {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
