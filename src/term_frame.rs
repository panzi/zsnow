use crate::{color::Rgb, draw_mode::DrawMode, rgb_image::RgbImage, size2d::Size2D, termio::TermIO};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermFrame {
    data: Box<[TermChar]>,
    size: Size2D,
}

impl TermFrame {
    #[inline]
    pub fn new(size: Size2D) -> Self {
        let data = vec![TermChar::default(); size.width * size.height].into_boxed_slice();
        Self { data, size }
    }

    #[inline]
    pub fn data(&self) -> &[TermChar] {
        &self.data
    }

    #[inline]
    pub fn size(&self) -> &Size2D {
        &self.size
    }

    pub fn draw(&mut self, column: usize, row: usize, image: &RgbImage, draw_mode: DrawMode) {
        match draw_mode {
            DrawMode::HalfBlock => self.draw_half_block(column, row, image),
            DrawMode::TwoByThree => self.draw_two_by_three(column, row, image),
            DrawMode::Braille => self.draw_braille(column, row, image),
        }
    }

    pub fn draw_half_block(&mut self, column: usize, row: usize, image: &RgbImage) {
        if column >= self.size.width || row >= self.size.height {
            return;
        }

        let columns = image.size().width.div_ceil(2);
        let rows = image.size().height.div_ceil(2);

        let columns = if column + columns > self.size.width {
            self.size.width - column
        } else {
            columns
        };

        let rows = if row + rows > self.size.height {
            self.size.height - row
        } else {
            rows
        };

        for row in row..row + rows {
            let index = row * self.size.width + column;
            let term_row = &mut self.data[index..index + columns];

            // TODO
        }

        unimplemented!()
    }

    pub fn draw_two_by_three(&mut self, column: usize, row: usize, image: &RgbImage) {
        unimplemented!()
    }

    pub fn draw_braille(&mut self, column: usize, row: usize, image: &RgbImage) {
        unimplemented!()
    }

    pub fn diff_redraw(&self, prev_frame: &TermFrame, termio: &mut TermIO) -> std::io::Result<()> {
        if self.size != prev_frame.size {
            return self.full_redraw(termio);
        }

        if self.data.is_empty() {
            return Ok(());
        }

        let mut curr_fg = None;
        let mut curr_bg = None;
        let mut prev_x = 0;
        let mut prev_y = 0;

        let mut char_buf = [0u8; char::MAX_LEN_UTF8];

        for (y, (row, prev_row)) in self.data.chunks(self.size.height).zip(prev_frame.data.chunks(prev_frame.size.height)).enumerate() {
            for (x, (c, prev_c)) in row.iter().zip(prev_row.iter()).enumerate() {
                if c != prev_c {
                    if x == 0 && prev_y + 1 == y {
                        termio.write_str("\n")?;
                    } else if prev_y == y {
                        if prev_x + 1 == x {
                            // already at correct position
                        } else if prev_x + 1 < x {
                            termio.move_cursor_forward((x - (prev_x + 1)) as u32)?;
                        } else {
                            termio.move_cursor_back(((prev_x + 1) - x) as u32)?;
                        }
                    } else {
                        if x > u32::MAX as usize || y > u32::MAX as usize {
                            break;
                        }
                        termio.move_cursor(x as u32, y as u32)?;
                    }

                    prev_x = x;
                    prev_y = y;

                    if Some(c.fg) != curr_fg {
                        termio.fg_rgb(c.fg)?;
                        curr_fg = Some(c.fg);
                    }

                    if Some(c.bg) != curr_bg {
                        termio.bg_rgb(c.bg)?;
                        curr_bg = Some(c.bg);
                    }

                    termio.write_str(c.c.encode_utf8(&mut char_buf))?;
                }
            }
        }

        Ok(())
    }

    pub fn full_redraw(&self, termio: &mut TermIO) -> std::io::Result<()> {
        termio.move_cursor(0, 0)?;

        if self.data.is_empty() {
            return Ok(());
        }

        let mut curr_fg = self.data[0].fg;
        let mut curr_bg = self.data[0].bg;

        termio.fg_rgb(curr_fg)?;
        termio.bg_rgb(curr_bg)?;

        let mut char_buf = [0u8; char::MAX_LEN_UTF8];
        let mut first = true;

        for row in self.data.chunks(self.size.height) {
            if first {
                first = false;
            } else {
                termio.write_str("\n")?;
            }

            for &TermChar { c, fg, bg } in row {
                if fg != curr_fg {
                    termio.fg_rgb(fg)?;
                    curr_fg = fg;
                }

                if bg != curr_bg {
                    termio.bg_rgb(bg)?;
                    curr_bg = bg;
                }

                termio.write_str(c.encode_utf8(&mut char_buf))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TermChar {
    pub c: char,
    pub fg: Rgb,
    pub bg: Rgb,
}

impl Default for TermChar {
    #[inline]
    fn default() -> Self {
        Self {
            c: ' ',
            fg: Rgb::default(),
            bg: Rgb::default(),
        }
    }
}

#[inline]
pub fn get_bits_from_half_block(c: char) -> u32 {
    match c {
        '\u{2580}' => 0b01, // upper half block
        '\u{2584}' => 0b10, // lower half block
        '\u{2588}' => 0b11, // full block
        _ => 0,
    }
}

#[inline]
pub fn get_bits_from_two_by_three(c: char) -> u32 {
    match c {
        '\u{2588}' => 0b11_11_11, // full block
        _ if c >= '\u{1FB00}' && c <= '\u{1FB3B}' => {
            let c: u32 = c.into();
            c + 1 - 0x1FB00
        },
        _ => 0,
    }
}

#[inline]
pub fn get_bits_from_braille(c: char) -> u32 {
    match c {
        _ if c >= '\u{2800}' && c <= '\u{28FF}' => {
            let c: u32 = c.into();
            c - 0x2800
        },
        _ => 0,
    }
}

#[inline]
pub fn get_half_block_from_bits(bits: u32) -> char {
    match bits & 0b11 {
        0b01 => '\u{2580}',
        0b10 => '\u{2584}',
        0b11 => '\u{2580}',
        _ => ' ',
    }
}

#[inline]
pub fn get_two_by_three_from_bits(bits: u32) -> char {
    match bits & 0b11_11_11 {
        0 => ' ',
        0b11_11_11 => '\u{2588}',
        bits => {
            unsafe { char::from_u32_unchecked(0x1FB00 + bits - 1) }
        },
    }
}

#[inline]
pub fn get_braille_from_bits(bits: u32) -> char {
    unsafe { char::from_u32_unchecked(0x2800 + (bits & 0xFF)) }
}

#[inline]
pub fn get_half_block_mask(y: u32) -> u32 {
    if y > 1 {
        return 0;
    }

    1 << y
}

#[inline]
pub fn get_two_by_three_mask(x: u32, y: u32) -> u32 {
    if x > 1 || y > 2 {
        return 0;
    }

    1 << (y * 2 + x)
}

#[inline]
pub fn get_braille_mask(x: u32, y: u32) -> u32 {
    if x > 1 || y > 3 {
        return 0;
    }

    BRAILLE_MASK[(y * 2 + x) as usize]
}

#[inline]
pub fn get_pixel_from_half_block(bits: u32, y: u32) -> bool {
    (bits & (1 << y)) != 0
}

#[inline]
pub fn get_pixel_from_two_by_three(bits: u32, x: u32, y: u32) -> bool {
    bits & (1 << (y * 2 + x)) != 0
}

const BRAILLE_MASK: [u32; 8] = [
    0x01, 0x08,
    0x02, 0x10,
    0x04, 0x20,
    0x40, 0x80,
];

#[inline]
pub fn get_pixel_from_braille(bits: u32, x: u32, y: u32) -> bool {
    if x > 1 || y > 3 {
        return false;
    }

    (BRAILLE_MASK[(y * 2 + x) as usize] & bits) != 0
}

#[inline]
pub fn set_pixel_in_half_block(bits: u32, y: u32) -> u32 {
    bits | (1 << y)
}

#[inline]
pub fn set_pixel_in_two_by_three(bits: u32, x: u32, y: u32) -> u32 {
    bits | (1 << (y * 2 + x))
}

#[inline]
pub fn set_pixel_in_braille(bits: u32, x: u32, y: u32) -> u32 {
    if x > 1 || y > 3 {
        return bits;
    }

    bits | BRAILLE_MASK[(y * 2 + x) as usize]
}

#[inline]
pub fn clear_pixel_in_half_block(bits: u32, y: u32) -> u32 {
    bits & !(1 << y)
}

#[inline]
pub fn clear_pixel_in_two_by_three(bits: u32, x: u32, y: u32) -> u32 {
    bits & !(1 << (y * 2 + x))
}

#[inline]
pub fn clear_pixel_in_braille(bits: u32, x: u32, y: u32) -> u32 {
    if x > 1 || y > 3 {
        return bits;
    }

    bits & !BRAILLE_MASK[(y * 2 + x) as usize]
}
