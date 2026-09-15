use std::str::FromStr;

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

#[derive(Debug)]
pub struct ParseDrawModeError;

impl std::error::Error for ParseDrawModeError {}

impl std::fmt::Display for ParseDrawModeError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "illegal draw mode".fmt(f)
    }
}

impl FromStr for DrawMode {
    type Err = ParseDrawModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("halfblock") || s.eq_ignore_ascii_case("half-block") || s.eq_ignore_ascii_case("half_block") {
            Ok(DrawMode::HalfBlock)
        } else if s.eq_ignore_ascii_case("2by3") || s.eq_ignore_ascii_case("twobythree") || s.eq_ignore_ascii_case("two-by-three") || s.eq_ignore_ascii_case("two_by_three") {
            Ok(DrawMode::TwoByThree)
        } else if s.eq_ignore_ascii_case("braille") {
            Ok(DrawMode::Braille)
        } else {
            Err(ParseDrawModeError)
        }
    }
}
