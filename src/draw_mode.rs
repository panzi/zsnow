use crate::size2d::Size2D;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawMode {
    HalfBlock,
    TwoByThree,
    Braille,
}

impl Default for DrawMode {
    #[inline]
    fn default() -> Self {
        Self::HalfBlock
    }
}

impl DrawMode {
    #[inline]
    pub const fn size(&self) -> Size2D {
        match self {
            Self::HalfBlock  => Size2D { width: 1, height: 2 },
            Self::TwoByThree => Size2D { width: 2, height: 3 },
            Self::Braille    => Size2D { width: 2, height: 4 },
        }
    }
}
