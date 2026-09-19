use std::ops::Mul;

use crate::point3d::Point3D;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix3d(pub [f32; 9]);

impl Matrix3d {
    pub const UNIT: Self = Self([
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]);

    #[inline]
    pub fn from_rotation_x(rad: f32) -> Self {
        let rad_sin = rad.sin();
        let rad_cos = rad.cos();

        Self([
            1.0, 0.0, 0.0,
            0.0, rad_cos, -rad_sin,
            0.0, rad_sin, rad_cos,
        ])
    }

    #[inline]
    pub fn from_rotation_y(rad: f32) -> Self {
        let rad_sin = rad.sin();
        let rad_cos = rad.cos();

        Self([
            rad_cos, 0.0, rad_sin,
            0.0, 1.0, 0.0,
            -rad_sin, 0.0, rad_cos,
        ])
    }

    #[inline]
    pub fn from_rotation_z(rad: f32) -> Self {
        let rad_sin = rad.sin();
        let rad_cos = rad.cos();

        Self([
            rad_cos, -rad_sin, 0.0,
            rad_sin, rad_cos, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    /// Extrinsic rotation
    #[inline]
    pub fn from_rotation(rad_x: f32, rad_y: f32, rad_z: f32) -> Self {
        let sin_x = rad_x.sin();
        let cos_x = rad_x.cos();
        let sin_y = rad_y.sin();
        let cos_y = rad_y.cos();
        let sin_z = rad_z.sin();
        let cos_z = rad_z.cos();

        Self([
            cos_y * cos_z, -cos_y * sin_z, sin_y,
            cos_x * sin_z + sin_x * sin_y * cos_z, cos_x * cos_z - sin_x * sin_y * sin_z, -sin_x * cos_y,
            sin_x * sin_z - cos_x * sin_y * cos_z, sin_x * cos_z + cos_x * sin_y * sin_z, cos_x * cos_y,
        ])
    }

    #[inline]
    pub fn from_scale(sx: f32, sy: f32, sz: f32) -> Self {
        Self([
            sx, 0.0, 0.0,
            0.0, sy, 0.0,
            0.0, 0.0, sz,
        ])
    }
}

// 0 1 2
// 3 4 5
// 6 7 8

impl Mul for &Matrix3d {
    type Output = Matrix3d;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = &self.0;
        let rhs = &rhs.0;

        Matrix3d([
            lhs[0] * rhs[0] + lhs[1] * rhs[3] + lhs[2] * rhs[6],
            lhs[0] * rhs[1] + lhs[1] * rhs[4] + lhs[2] * rhs[7],
            lhs[0] * rhs[2] + lhs[1] * rhs[5] + lhs[2] * rhs[8],

            lhs[3] * rhs[0] + lhs[4] * rhs[3] + lhs[5] * rhs[6],
            lhs[3] * rhs[1] + lhs[4] * rhs[4] + lhs[5] * rhs[7],
            lhs[3] * rhs[2] + lhs[4] * rhs[5] + lhs[5] * rhs[8],

            lhs[6] * rhs[0] + lhs[7] * rhs[3] + lhs[8] * rhs[6],
            lhs[6] * rhs[1] + lhs[7] * rhs[4] + lhs[8] * rhs[7],
            lhs[6] * rhs[2] + lhs[7] * rhs[5] + lhs[8] * rhs[8],
        ])
    }
}

impl Mul<&Point3D> for &Matrix3d {
    type Output = Point3D;

    #[inline]
    fn mul(self, rhs: &Point3D) -> Self::Output {
        let lhs = &self.0;

        Point3D {
            x: lhs[0] * rhs.x + lhs[1] * rhs.y + lhs[2] * rhs.z,
            y: lhs[3] * rhs.x + lhs[4] * rhs.y + lhs[5] * rhs.z,
            z: lhs[6] * rhs.x + lhs[7] * rhs.y + lhs[8] * rhs.z,
        }
    }
}

impl Mul<&Matrix3d> for &Point3D {
    type Output = Point3D;

    #[inline]
    fn mul(self, rhs: &Matrix3d) -> Self::Output {
        let rhs = &rhs.0;

        Point3D {
            x: self.x * rhs[0] + self.y * rhs[3] + self.z * rhs[6],
            y: self.x * rhs[1] + self.y * rhs[4] + self.z * rhs[7],
            z: self.x * rhs[2] + self.y * rhs[5] + self.z * rhs[8],
        }
    }
}

impl Mul<&(f32, f32, f32)> for &Matrix3d {
    type Output = (f32, f32, f32);

    #[inline]
    fn mul(self, &(x, y, z): &(f32, f32, f32)) -> Self::Output {
        let lhs = &self.0;

        (
            lhs[0] * x + lhs[1] * y + lhs[2] * z,
            lhs[3] * x + lhs[4] * y + lhs[5] * z,
            lhs[6] * x + lhs[7] * y + lhs[8] * z,
        )
    }
}

impl Mul<&Matrix3d> for &(f32, f32, f32) {
    type Output = (f32, f32, f32);

    #[inline]
    fn mul(self, rhs: &Matrix3d) -> Self::Output {
        let &(x, y, z) = self;
        let rhs = &rhs.0;

        (
            x * rhs[0] + y * rhs[3] + z * rhs[6],
            x * rhs[1] + y * rhs[4] + z * rhs[7],
            x * rhs[2] + y * rhs[5] + z * rhs[8],
        )
    }
}

impl Mul<&[f32; 3]> for &Matrix3d {
    type Output = [f32; 3];

    #[inline]
    fn mul(self, &[x, y, z]: &[f32; 3]) -> Self::Output {
        let lhs = &self.0;

        [
            lhs[0] * x + lhs[1] * y + lhs[2] * z,
            lhs[3] * x + lhs[4] * y + lhs[5] * z,
            lhs[6] * x + lhs[7] * y + lhs[8] * z,
        ]
    }
}

impl Mul<&Matrix3d> for &[f32; 3] {
    type Output = [f32; 3];

    #[inline]
    fn mul(self, rhs: &Matrix3d) -> Self::Output {
        let &[x, y, z] = self;
        let rhs = &rhs.0;

        [
            x * rhs[0] + y * rhs[3] + z * rhs[6],
            x * rhs[1] + y * rhs[4] + z * rhs[7],
            x * rhs[2] + y * rhs[5] + z * rhs[8],
        ]
    }
}
