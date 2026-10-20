use std::{str::FromStr, time::Duration};

use crate::{parse::{parse_gust, parse_gusts, skipws}, point3d::Point3D};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gust {
    Linear { velocity: Point3D },
    Radial { center: Point3D, strength: f32 },
}

impl std::fmt::Display for Gust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gust::Linear { velocity } => write!(f, "linear {velocity}"),
            Gust::Radial { center, strength } => write!(f, "radial {center} {strength}")
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct GustParseError;

impl std::fmt::Display for GustParseError {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "illegal gust".fmt(f)
    }
}

impl std::error::Error for GustParseError {}

impl FromStr for Gust {
    type Err = GustParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, gust) = parse_gust(s)?;
        let s = skipws(s);

        if !s.is_empty() {
            return Err(GustParseError);
        }

        Ok(gust)
    }
}

#[derive(Debug, Clone)]
pub struct Gusts(pub Vec<(Duration, Gust)>);

impl From<Vec<(Duration, Gust)>> for Gusts {
    #[inline]
    fn from(value: Vec<(Duration, Gust)>) -> Self {
        Self(value)
    }
}

impl From<Gusts> for Vec<(Duration, Gust)> {
    #[inline]
    fn from(value: Gusts) -> Self {
        value.0
    }
}

impl FromStr for Gusts {
    type Err = GustParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, gusts) = parse_gusts(s)?;
        let s = skipws(s);

        if !s.is_empty() {
            return Err(GustParseError);
        }

        Ok(Gusts(gusts))
    }
}
