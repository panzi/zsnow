use crate::{color::Rgb, draw_mode::DrawMode, rect::Rect, rgb_image::RgbImage, size2d::Size2D, termio::TermIO};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermFrame {
    data: Vec<TermChar>,
    size: Size2D,
}

impl TermFrame {
    #[inline]
    pub fn new(size: Size2D) -> Self {
        let data = vec![TermChar::default(); size.width * size.height];
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

    #[inline]
    pub fn resize(&mut self, size: &Size2D) {
        self.data.resize(size.width * size.height, TermChar::default());
        self.size = *size;
    }

    #[inline]
    pub fn fill(&mut self, value: TermChar) {
        self.data.fill(value);
    }

    pub fn fill_rect(&mut self, rect: &Rect, value: &TermChar) {
        let &Rect { x, y, mut width, mut height } = rect;

        let y = if y < 0 {
            if -y as usize >= height {
                return;
            }

            height -= -y as usize;
            0
        } else {
            y as usize
        };

        if y >= self.size.height {
            return;
        }

        if y + height > self.size.height {
            height = self.size.height - y;
        }

        let x = if x < 0 {
            if -x as usize >= width {
                return;
            }

            width -= -x as usize;
            0
        } else {
            x as usize
        };

        if x >= self.size.width {
            return;
        }

        if x + width > self.size.width {
            width = self.size.width - x;
        }

        if x == 0 && width == self.size.width {
            let start = y * self.size.width;
            self.data[start..start + self.size.width * height].fill(*value);
        } else {
            for y in y..y + height {
                let start = y * self.size.width + x;
                self.data[start..start + width].fill(*value);
            }
        }
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

        let columns = image.size().width;
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

        for term_row_index in row..row + rows {
            let term_slice_index = term_row_index * self.size.width + column;
            let term_row = &mut self.data[term_slice_index..term_slice_index + columns];

            let line_index = term_row_index * 2;
            let line1 = image.get_line(line_index);
            let line2 = image.get_line(line_index + 1);
            let mut curr_fg = None;
            let mut curr_bg = None;

            for ((term_cell, &color1), &color2) in term_row.iter_mut().zip(line1).zip(line2) {
                if color1 == color2 {
                    term_cell.fg = color1;
                    term_cell.bg = curr_bg.unwrap_or(color1);
                    term_cell.c = FULL_BLOCK;
                } else if Some(color1) == curr_fg || Some(color2) == curr_bg {
                    term_cell.fg = color1;
                    term_cell.bg = color2;
                    term_cell.c = UPPER_HALF_BLOCK;
                } else /* if Some(color2) == curr_fg || Some(color1) == curr_bg */ {
                    term_cell.fg = color2;
                    term_cell.bg = color1;
                    term_cell.c = LOWER_HALF_BLOCK;
                }

                curr_fg = Some(term_cell.fg);
                curr_bg = Some(term_cell.bg);
            }
        }
    }

    pub fn draw_two_by_three(&mut self, column: usize, row: usize, image: &RgbImage) {
        unimplemented!()
    }

    pub fn draw_braille(&mut self, column: usize, row: usize, image: &RgbImage) {
        unimplemented!()
    }

    #[inline]
    pub fn get_row(&self, index: usize) -> &[TermChar] {
        let term_slice_index = index * self.size.width;
        &self.data[term_slice_index..term_slice_index + self.size.width]
    }

    #[inline]
    pub fn get_row_mut(&mut self, index: usize) -> &mut [TermChar] {
        let term_slice_index = index * self.size.width;
        &mut self.data[term_slice_index..term_slice_index + self.size.width]
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

const FULL_BLOCK: char = '\u{2588}';
const UPPER_HALF_BLOCK: char = '\u{2580}';
const LOWER_HALF_BLOCK: char = '\u{2584}';

#[inline]
pub fn get_bits_from_half_block(c: char) -> u32 {
    match c {
        self::UPPER_HALF_BLOCK => 0b01,
        self::LOWER_HALF_BLOCK => 0b10,
        self::FULL_BLOCK => 0b11,
        _ => 0,
    }
}

#[inline]
pub fn get_bits_from_two_by_three(c: char) -> u32 {
    match c {
        self::FULL_BLOCK => 0b11_11_11,
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
        0b01 => UPPER_HALF_BLOCK,
        0b10 => LOWER_HALF_BLOCK,
        0b11 => FULL_BLOCK,
        _ => ' ',
    }
}

#[inline]
pub fn get_two_by_three_from_bits(bits: u32) -> char {
    match bits & 0b11_11_11 {
        0 => ' ',
        0b11_11_11 => FULL_BLOCK,
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
