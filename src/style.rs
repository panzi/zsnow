use crate::ansi_codes::{BOLD, DOUBLY_UNDERLINE, FAINT, ITALIC, NORMAL_INTENSITY, NOT_ITALIC, NOT_UNDERLINE, UNDERLINE};

pub trait Style {
    fn write(&self, write: &mut impl std::io::Write) -> std::io::Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Normal,
    Bold,
    Faint,
}

impl Default for FontWeight {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

impl Style for FontWeight {
    #[inline]
    fn write(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            FontWeight::Normal => write.write_all(NORMAL_INTENSITY),
            FontWeight::Bold => write.write_all(BOLD),
            FontWeight::Faint => write.write_all(FAINT),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecoration {
    None,
    Underline,
    DoublyUnderline,
}

impl Default for TextDecoration {
    #[inline]
    fn default() -> Self {
        Self::None
    }
}

impl Style for TextDecoration {
    #[inline]
    fn write(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            TextDecoration::None => write.write_all(NOT_UNDERLINE),
            TextDecoration::Underline => write.write_all(UNDERLINE),
            TextDecoration::DoublyUnderline => write.write_all(DOUBLY_UNDERLINE),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Normal,
    Italic,
}

impl Default for FontStyle {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}

impl Style for FontStyle {
    #[inline]
    fn write(&self, write: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            FontStyle::Normal => write.write_all(NOT_ITALIC),
            FontStyle::Italic => write.write_all(ITALIC),
        }
    }
}
