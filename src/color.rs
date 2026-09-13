#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color16 {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color16 {
    pub fn parse(value: &str) -> Option<Self> {
        if value.eq_ignore_ascii_case("black") {
            Some(Color16::Black)
        } else if value.eq_ignore_ascii_case("red") {
            Some(Color16::Red)
        } else if value.eq_ignore_ascii_case("green") {
            Some(Color16::Green)
        } else if value.eq_ignore_ascii_case("yellow") {
            Some(Color16::Yellow)
        } else if value.eq_ignore_ascii_case("blue") {
            Some(Color16::Blue)
        } else if value.eq_ignore_ascii_case("magenta") {
            Some(Color16::Magenta)
        } else if value.eq_ignore_ascii_case("cyan") {
            Some(Color16::Cyan)
        } else if value.eq_ignore_ascii_case("white") {
            Some(Color16::White)
        } else if value.eq_ignore_ascii_case("brightblack") || value.eq_ignore_ascii_case("grey") || value.eq_ignore_ascii_case("gray") {
            Some(Color16::BrightBlack)
        } else if value.eq_ignore_ascii_case("brightred") {
            Some(Color16::BrightRed)
        } else if value.eq_ignore_ascii_case("brightgreen") {
            Some(Color16::BrightGreen)
        } else if value.eq_ignore_ascii_case("brightyellow") {
            Some(Color16::BrightYellow)
        } else if value.eq_ignore_ascii_case("brightblue") {
            Some(Color16::BrightBlue)
        } else if value.eq_ignore_ascii_case("brightmagenta") {
            Some(Color16::BrightMagenta)
        } else if value.eq_ignore_ascii_case("brightcyan") {
            Some(Color16::BrightCyan)
        } else if value.eq_ignore_ascii_case("brightwhite") {
            Some(Color16::BrightWhite)
        } else {
            None
        }
    }

    #[inline]
    pub fn fg(&self) -> &'static [u8] {
        match *self {
            Self::Black         => b"\x1B[30m",
            Self::Red           => b"\x1B[31m",
            Self::Green         => b"\x1B[32m",
            Self::Yellow        => b"\x1B[33m",
            Self::Blue          => b"\x1B[34m",
            Self::Magenta       => b"\x1B[35m",
            Self::Cyan          => b"\x1B[36m",
            Self::White         => b"\x1B[37m",
            Self::BrightBlack   => b"\x1B[90m",
            Self::BrightRed     => b"\x1B[91m",
            Self::BrightGreen   => b"\x1B[92m",
            Self::BrightYellow  => b"\x1B[93m",
            Self::BrightBlue    => b"\x1B[94m",
            Self::BrightMagenta => b"\x1B[95m",
            Self::BrightCyan    => b"\x1B[96m",
            Self::BrightWhite   => b"\x1B[97m",
        }
    }

    #[inline]
    pub fn bg(&self) -> &'static [u8] {
        match *self {
            Self::Black         => b"\x1B[40m",
            Self::Red           => b"\x1B[41m",
            Self::Green         => b"\x1B[42m",
            Self::Yellow        => b"\x1B[43m",
            Self::Blue          => b"\x1B[44m",
            Self::Magenta       => b"\x1B[45m",
            Self::Cyan          => b"\x1B[46m",
            Self::White         => b"\x1B[47m",
            Self::BrightBlack   => b"\x1B[100m",
            Self::BrightRed     => b"\x1B[101m",
            Self::BrightGreen   => b"\x1B[102m",
            Self::BrightYellow  => b"\x1B[103m",
            Self::BrightBlue    => b"\x1B[104m",
            Self::BrightMagenta => b"\x1B[105m",
            Self::BrightCyan    => b"\x1B[106m",
            Self::BrightWhite   => b"\x1B[107m",
        }
    }

    #[inline]
    pub fn to_color(&self) -> Color {
        Color::Color16(*self)
    }

    #[inline]
    pub fn into_color(self) -> Color {
        Color::Color16(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub const fn from_u32(color: u32) -> Self {
        Self {
            r: ((color >> 16) & 0xFF) as u8,
            g: ((color >>  8) & 0xFF) as u8,
            b: (color         & 0xFF) as u8,
        }
    }

    pub const fn from_hsl(&Hsl { h, s, l }: &Hsl) -> Self {
        // See: https://stackoverflow.com/a/9493060/277767

        #[inline]
        const fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
            if t < 0.0 { t += 1.0; }
            if t > 1.0 { t -= 1.0; }
            if t < 1.0/6.0 { return p + (q - p) * 6.0 * t; }
            if t < 1.0/2.0 { return q; }
            if t < 2.0/3.0 { return p + (q - p) * (2.0/3.0 - t) * 6.0; }

            p
        }

        let r;
        let g;
        let b;

        if s == 0.0 {
            // achromatic
            r = l;
            g = l;
            b = l;
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            r = hue_to_rgb(p, q, h + 1.0/3.0);
            g = hue_to_rgb(p, q, h);
            b = hue_to_rgb(p, q, h - 1.0/3.0);
        }

        Rgb {
            r: (r * 256.0).floor().min(255.0) as u8,
            g: (g * 256.0).floor().min(255.0) as u8,
            b: (b * 256.0).floor().min(255.0) as u8,
        }
    }

    #[inline]
    pub const fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    #[inline]
    pub const fn to_color(&self) -> Color {
        Color::Rgb { r: self.r, g: self.g, b: self. b }
    }

    #[inline]
    pub const fn to_hsl(&self) -> Hsl {
        Hsl::from_rgb(*self)
    }

    #[inline]
    pub fn blend(&self, other: Rgb, alpha: u8) -> Rgb {
        let mut rgb = self.clone();
        rgb.blend_assign(other, alpha);
        rgb
    }

    #[inline]
    pub fn blend_assign(&mut self, other: Rgb, alpha: u8) {
        let alpha = alpha as u32;
        let inv_alpha = 0xFF - alpha;
        self.r = ((self.r as u32 * inv_alpha + other.r as u32 * alpha) / 0xFF) as u8;
        self.g = ((self.g as u32 * inv_alpha + other.g as u32 * alpha) / 0xFF) as u8;
        self.b = ((self.b as u32 * inv_alpha + other.b as u32 * alpha) / 0xFF) as u8;
    }
}

impl std::fmt::Display for Rgb {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Rgb { r, g, b } = self;
        write!(f, "#{r:02X}{g:02X}{b:02X}")
    }
}

impl From<u32> for Rgb {
    #[inline]
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}

impl From<Rgb> for u32 {
    #[inline]
    fn from(value: Rgb) -> Self {
        value.to_u32()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Hsl {
    pub h: f32,
    pub s: f32,
    pub l: f32,
}

impl Hsl {
    #[inline]
    pub const fn new(h: f32, s: f32, l: f32) -> Self {
        Self { h, s, l }
    }

    pub const fn from_rgb(Rgb { r, g, b }: Rgb) -> Self {
        // See: https://stackoverflow.com/a/9493060/277767

        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;
        let vmax = r.max(g).max(b);
        let vmin = r.min(g).min(b);
        let l = (vmin + vmax) * 0.5;

        if vmax == vmin {
            return Hsl { h: 0.0, s: 0.0, l };
        }

        let d = vmax - vmin;
        let s = if l > 0.5 {
            d / (2.0 - vmax - vmin)
        } else {
            d / (vmax + vmin)
        };

        let h = if vmax == r {
            let h = (g - b) / d;
            if g < b { h + 6.0 } else { h }
        } else if vmax == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };

        let h = h / 6.0;

        Hsl { h, s, l }
    }

    #[inline]
    pub const fn to_rgb(&self) -> Rgb {
        Rgb::from_hsl(self)
    }

    #[inline]
    pub fn blend(&self, other: Hsl, alpha: f32) -> Hsl {
        let mut rgb = self.clone();
        rgb.blend_assign(other, alpha);
        rgb
    }

    #[inline]
    pub fn blend_assign(&mut self, other: Hsl, alpha: f32) {
        let inv_alpha = 1.0 - alpha;
        self.h = self.h * inv_alpha + other.h * alpha;
        self.s = self.s * inv_alpha + other.s * alpha;
        self.l = self.l * inv_alpha + other.l * alpha;
    }
}

impl std::fmt::Display for Hsl {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Hsl { h, s, l } = self;
        write!(f, "hsl({h:?} {s:?} {l:?})")
    }
}

impl From<&Hsl> for Rgb {
    #[inline]
    fn from(value: &Hsl) -> Self {
        Rgb::from_hsl(value)
    }
}

impl From<Rgb> for Hsl {
    #[inline]
    fn from(value: Rgb) -> Self {
        Hsl::from_rgb(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Default,
    Rgb { r: u8, g: u8, b: u8 },
    Color16(Color16),
}

impl Color {
    #[inline]
    pub const fn from_u32(color: u32) -> Self {
        Self::Rgb {
            r: ((color >> 16) & 0xFF) as u8,
            g: ((color >>  8) & 0xFF) as u8,
            b: (color         & 0xFF) as u8,
        }
    }

    #[inline]
    pub fn is_default(&self) -> bool {
        matches!(self, Color::Default)
    }

    #[inline]
    pub fn is_rgb(&self) -> bool {
        matches!(self, Color::Rgb { .. })
    }

    #[inline]
    pub fn is_color16(&self) -> bool {
        matches!(self, Color::Color16(..))
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

impl From<Rgb> for Color {
    #[inline]
    fn from(Rgb { r, g, b }: Rgb) -> Self {
        Self::Rgb { r, g, b }
    }
}

impl From<Color16> for Color {
    #[inline]
    fn from(color16: Color16) -> Self {
        Self::Color16(color16)
    }
}
