use std::{ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign}, str::FromStr};


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

impl std::fmt::Display for Point3D {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct IllegalPoint3D;

impl std::error::Error for IllegalPoint3D {
}

impl std::fmt::Display for IllegalPoint3D {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "illegal value for Point3D".fmt(f)
    }
}

impl FromStr for Point3D {
    type Err = IllegalPoint3D;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = skipws(s);

        let (s, v1) = parse_f32(s)?;

        if s.starts_with(|c: char| c.is_whitespace()) {
            // x y z
            let x = v1;
            let s = skipws(s);
            let (s, y) = parse_f32(s)?;

            if !s.starts_with(|c: char| c.is_whitespace()) {
                return Err(IllegalPoint3D);
            }

            let s = skipws(s);
            let (s, z) = parse_f32(s)?;
            let s = skipws(s);

            if !s.is_empty() {
                return Err(IllegalPoint3D);
            }

            Ok(Point3D { x, y, z })
        } else if s.starts_with("deg") {
            // degree
            let s = skipws(&s[3..]);

            if !s.is_empty() {
                return Err(IllegalPoint3D);
            }

            let alpha_deg = v1;

            // ensure exact values
            if alpha_deg == 180.0 {
                Ok(Point3D {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0
                })
            } else if alpha_deg == 90.0 {
                Ok(Point3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0
                })
            } else if alpha_deg == -90.0 {
                Ok(Point3D {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0
                })
            } else if alpha_deg == 0.0 {
                Ok(Point3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0
                })
            } else {
                let alpha_rad = alpha_deg * std::f32::consts::PI / 180.0;

                Ok(Point3D {
                    x: alpha_rad.cos(),
                    y: alpha_rad.sin(),
                    z: 0.0
                })
            }
        } else if s.starts_with("rad") {
            // radiants
            let s = skipws(&s[3..]);

            if !s.is_empty() {
                return Err(IllegalPoint3D);
            }

            let alpha = v1;

            Ok(Point3D {
                x: alpha.cos(),
                y: alpha.sin(),
                z: 0.0
            })
        } else {
            return Err(IllegalPoint3D);
        }
    }
}

fn skipws(input: &str) -> &str {
    let endpos = input.find(|c: char| !c.is_whitespace()).unwrap_or(input.len());

    &input[endpos..]
}

fn parse_f32<E: Default>(input: &str) -> Result<(&str, f32), E> {
    let mut endpos = 0;

    if input.starts_with('-') || input.starts_with('+') {
        endpos += 1;
    }

    endpos = if let Some(index) = input[endpos..].find(|c: char| !c.is_ascii_digit()) {
        endpos + index
    } else { input.len() };

    let tail = &input[endpos..];

    if tail.starts_with('.') {
        endpos += 1;
        endpos = if let Some(index) = input[endpos..].find(|c: char| !c.is_ascii_digit()) {
            endpos + index
        } else { input.len() };
    }

    let Ok(value) = input[..endpos].parse() else {
        return Err(E::default());
    };

    Ok((&input[endpos..], value))

}
