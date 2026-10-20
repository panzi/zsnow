use std::{str::FromStr, time::Duration};

use crate::{point3d::Point3D, wind::Gust};

#[inline]
pub fn skipws(s: &str) -> &str {
    let endpos = s.find(|c: char| !c.is_whitespace()).unwrap_or(s.len());
    &s[endpos..]
}

pub fn parse_float<E: Default, F: FromStr>(input: &str) -> Result<(&str, F), E> {
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

#[inline]
pub fn parse_f32<E: Default>(input: &str) -> Result<(&str, f32), E> {
    parse_float(input)
}

#[inline]
fn at_word_border(s: &str) -> bool {
    s.is_empty() || s.starts_with(|c: char| !c.is_ascii_alphanumeric() && c != '_')
}

pub fn parse_point3d<E: Default>(s: &str) -> Result<(&str, Point3D), E> {
    let s = skipws(s);

    let (s, v1) = parse_f32(s)?;

    if s.starts_with(|c: char| c.is_whitespace()) {
        // x y z
        let x = v1;
        let s = skipws(s);
        let (s, y) = parse_f32(s)?;

        if !s.starts_with(|c: char| c.is_whitespace()) {
            return Err(E::default());
        }

        let s = skipws(s);
        let (s, z) = parse_f32(s)?;

        if !at_word_border(s) {
            return Err(E::default());
        }

        Ok((s, Point3D { x, y, z }))
    } else if s.starts_with("deg") {
        // degree
        let s = &s[3..];

        if !at_word_border(s) {
            return Err(E::default());
        }

        let alpha_deg = v1;

        // ensure exact values
        if alpha_deg == 180.0 {
            Ok((s, Point3D {
                x: -1.0,
                y: 0.0,
                z: 0.0
            }))
        } else if alpha_deg == 90.0 {
            Ok((s, Point3D {
                x: 0.0,
                y: 1.0,
                z: 0.0
            }))
        } else if alpha_deg == -90.0 {
            Ok((s, Point3D {
                x: 0.0,
                y: -1.0,
                z: 0.0
            }))
        } else if alpha_deg == 0.0 {
            Ok((s, Point3D {
                x: 1.0,
                y: 0.0,
                z: 0.0
            }))
        } else {
            let alpha_rad = alpha_deg * std::f32::consts::PI / 180.0;

            Ok((s, Point3D {
                x: alpha_rad.cos(),
                y: alpha_rad.sin(),
                z: 0.0
            }))
        }
    } else if s.starts_with("rad") {
        // radiants
        let s = &s[3..];

        if !at_word_border(s) {
            return Err(E::default());
        }

        let alpha = v1;

        Ok((s, Point3D {
            x: alpha.cos(),
            y: alpha.sin(),
            z: 0.0
        }))
    } else {
        return Err(E::default());
    }
}

fn find_word_end(s: &str) -> usize {
    s.find(|c: char| !c.is_ascii_alphanumeric() && c != '_').unwrap_or(s.len())
}

pub fn parse_gust<E: Default>(s: &str) -> Result<(&str, Gust), E> {
    let s = skipws(s);

    let endpos = find_word_end(s);

    if endpos == 0 {
        return Err(E::default());
    }

    let word = &s[..endpos];
    let s = skipws(&s[endpos..]);

    match word {
        "linear" | "l" => {
            let (s, velocity) = parse_point3d(s)?;
            Ok((s, Gust::Linear { velocity }))
        },
        "radial" | "r" => {
            let (s, center) = parse_point3d(s)?;
            let s = skipws(s);
            let (s, strength) = parse_f32(s)?;

            Ok((s, Gust::Radial { center, strength }))
        },
        _ => {
            Err(E::default())
        }
    }
}

pub fn parse_duration<E: Default>(s: &str) -> Result<(&str, Duration), E> {
    let s = skipws(s);
    let (s, value) = parse_float(s)?;
    let endpos = find_word_end(s);
    let word = &s[..endpos];
    let s = &s[endpos..];

    match word {
        "s" => {
            Ok((s, Duration::from_secs_f64(value)))
        }
        "ms" => {
            Ok((s, Duration::from_secs_f64(value * 0.001)))
        }
        "m" => {
            Ok((s, Duration::from_secs_f64(value * 60.0)))
        }
        "h" => {
            Ok((s, Duration::from_secs_f64(value * 1440.0)))
        }
        _ => {
            Err(E::default())
        }
    }
}

pub fn parse_gusts<E: Default>(s: &str) -> Result<(&str, Vec<(Duration, Gust)>), E> {
    let mut s = skipws(s);
    let mut gusts = Vec::new();

    if s.is_empty() {
        return Ok((s, gusts));
    }

    loop {
        let (s2, duration) = parse_duration(s)?;
        let s2 = skipws(s2);
        let (s2, gust) = parse_gust(s2)?;
        gusts.push((duration, gust));

        s = skipws(s2);

        if s.starts_with(",") {
            s = &s[1..];
        } else {
            break;
        }
    }

    Ok((s, gusts))
}
