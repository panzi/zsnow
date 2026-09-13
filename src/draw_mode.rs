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

const HALF_BLOCK_SIZE: Size2D = Size2D { width: 1, height: 2 };
const TWO_BY_THREE_SIZE: Size2D = Size2D { width: 2, height: 3 };
const BRAILLE_SIZE: Size2D = Size2D { width: 2, height: 4 };

impl DrawMode {
    #[inline]
    pub const fn size(&self) -> &Size2D {
        match self {
            Self::HalfBlock  => &HALF_BLOCK_SIZE,
            Self::TwoByThree => &TWO_BY_THREE_SIZE,
            Self::Braille    => &BRAILLE_SIZE,
        }
    }
}
