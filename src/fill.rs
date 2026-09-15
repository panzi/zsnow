use std::str::FromStr;

use crate::color::{ParseRgbError, Rgb};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fill {
    Solid(Rgb),
    VerticalGradient { top: Rgb, bottom: Rgb },
    HorizontalGradient { left: Rgb, right: Rgb },
}

impl Fill {
    #[inline]
    pub fn start(&self) -> Rgb {
        match self {
            Fill::Solid(color) => *color,
            Fill::VerticalGradient { top, .. } => *top,
            Fill::HorizontalGradient { left, .. } => *left,
        }
    }

    #[inline]
    pub fn end(&self) -> Rgb {
        match self {
            Fill::Solid(color) => *color,
            Fill::VerticalGradient { bottom, .. } => *bottom,
            Fill::HorizontalGradient { right, .. } => *right,
        }
    }
}

impl std::fmt::Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solid(color) => std::fmt::Display::fmt(color, f),
            Self::VerticalGradient { top, bottom } => write!(f, "vertical {top} {bottom}"),
            Self::HorizontalGradient { left, right } => write!(f, "horizontal {left} {right}"),
        }
    }
}

#[derive(Debug)]
pub struct ParseFillError {
    cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>
}

impl std::error::Error for ParseFillError {
    #[inline]
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Some(cause) = &self.cause {
            return Some(cause.as_ref());
        }

        None
    }
}

impl std::fmt::Display for ParseFillError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "illegal fill value".fmt(f)
    }
}

impl From<ParseRgbError> for ParseFillError {
    #[inline]
    fn from(value: ParseRgbError) -> Self {
        ParseFillError { cause: Some(Box::new(value)) }
    }
}

impl FromStr for Fill {
    type Err = ParseFillError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace().filter(|word| !word.is_empty());

        let Some(word) = words.next() else {
            return Err(ParseFillError { cause: None });
        };

        if word.eq_ignore_ascii_case("vertical") {
            let Some(top) = words.next() else {
                return Err(ParseFillError { cause: None });
            };

            let Some(bottom) = words.next() else {
                return Err(ParseFillError { cause: None });
            };

            if words.next().is_some() {
                return Err(ParseFillError { cause: None });
            }

            Ok(Fill::VerticalGradient {
                top: top.parse()?,
                bottom: bottom.parse()?,
            })
        } else if word.eq_ignore_ascii_case("horizontal") {
            let Some(left) = words.next() else {
                return Err(ParseFillError { cause: None });
            };

            let Some(right) = words.next() else {
                return Err(ParseFillError { cause: None });
            };

            if words.next().is_some() {
                return Err(ParseFillError { cause: None });
            }

            Ok(Fill::HorizontalGradient {
                left: left.parse()?,
                right: right.parse()?,
            })
        } else {
            if words.next().is_some() {
                return Err(ParseFillError { cause: None });
            }

            Ok(Fill::Solid(word.parse()?))
        }
    }
}
