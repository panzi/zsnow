use crate::termio::WindowSize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size2D {
    pub width: usize,
    pub height: usize,
}

impl std::fmt::Display for Size2D {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl From<WindowSize> for Size2D {
    #[inline]
    fn from(value: WindowSize) -> Self {
        Self {
            width: value.columns as usize,
            height: value.rows as usize,
        }
    }
}

impl std::ops::Mul for &Size2D {
    type Output = Size2D;

    #[inline]
    fn mul(self, rhs: &Size2D) -> Self::Output {
        Size2D {
            width: self.width * rhs.width,
            height: self.height * rhs.height,
        }
    }
}

impl std::ops::Mul<usize> for &Size2D {
    type Output = Size2D;

    #[inline]
    fn mul(self, rhs: usize) -> Self::Output {
        Size2D {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl std::ops::MulAssign<&Size2D> for Size2D {
    #[inline]
    fn mul_assign(&mut self, rhs: &Size2D) {
        self.width *= rhs.width;
        self.height *= rhs.height;
    }
}

impl std::ops::MulAssign<usize> for Size2D {
    #[inline]
    fn mul_assign(&mut self, rhs: usize) {
        self.width *= rhs;
        self.height *= rhs;
    }
}
