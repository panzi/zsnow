use std::{fmt::Write, slice::{Chunks, ChunksMut}};

use crate::{color::Rgb, rect::Rect, size2d::Size2D};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RgbImage {
    data: Vec<Rgb>,
    size: Size2D,
}

impl RgbImage {
    #[inline]
    pub fn new(size: Size2D, fill: Rgb) -> Self {
        let data = vec![fill; size.width * size.height];
        Self { data, size }
    }

    #[inline]
    pub fn size(&self) -> &Size2D {
        &self.size
    }

    #[inline]
    pub fn data(&self) -> &[Rgb] {
        &self.data
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut [Rgb] {
        &mut self.data
    }

    #[inline]
    pub fn get_pixel(&self, x: usize, y: usize) -> Rgb {
        if x >= self.size.width || y >= self.size.height {
            return Rgb::default();
        }

        self.data[self.size.width * y + x]
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }

        self.data[self.size.width * y + x] = color;
    }

    #[inline]
    pub fn set_pixel_alpha(&mut self, x: usize, y: usize, color: Rgb, alpha: u8) {
        if x >= self.size.width || y >= self.size.height {
            return;
        }

        let index = self.size.width * y + x;

        self.data[index].blend_assign(color, alpha);
    }

    #[inline]
    pub fn lines(&self) -> Chunks<'_, Rgb> {
        self.data.chunks(self.size.width)
    }

    #[inline]
    pub fn lines_mut(&mut self) -> ChunksMut<'_, Rgb> {
        self.data.chunks_mut(self.size.width)
    }

    #[inline]
    pub fn get_line(&self, y: usize) -> &[Rgb] {
        let slice_index = y * self.size.width;
        &self.data[slice_index..slice_index + self.size.width]
    }

    #[inline]
    pub fn get_line_mut(&mut self, y: usize) -> &mut [Rgb] {
        let slice_index = y * self.size.width;
        &mut self.data[slice_index..slice_index + self.size.width]
    }

    #[inline]
    pub fn fill(&mut self, color: Rgb) {
        self.data.fill(color);
    }

    pub fn vgradient(&mut self, top_color: Rgb, bottom_color: Rgb) {
        let height = self.size.height;
        for y in 0..height {
            let line = self.get_line_mut(y);
            line.fill(top_color.blend(bottom_color, (y * 255 / height) as u8));
        }
    }

    pub fn hgradient(&mut self, left_color: Rgb, right_color: Rgb) {
        let Size2D { width, height } = self.size;
        for x in 0..width {
            let color = left_color.blend(right_color, (x * 255 / width) as u8);
            for y in 0..height {
                self.data[y * width + x] = color;
            }
        }
    }

    #[inline]
    pub fn resize(&mut self, size: &Size2D) {
        self.data.resize(size.width * size.height, Rgb::default());
        self.size = *size;
    }

    pub fn fill_rect(&mut self, rect: &Rect, color: Rgb) {
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
            self.data[start..start + self.size.width * height].fill(color);
        } else {
            for y in y..y + height {
                let start = y * self.size.width + x;
                self.data[start..start + width].fill(color);
            }
        }
    }
}

impl std::fmt::Display for RgbImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_line(line: &[Rgb], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut iter = line.iter();

            if let Some(Rgb { r, g, b }) = iter.next() {
                write!(f, "{r:02X}{g:02X}{b:02X}")?;

                for Rgb { r, g, b } in iter {
                    write!(f, " {r:02X}{g:02X}{b:02X}")?;
                }
            }

            Ok(())
        }

        let mut iter = self.lines();

        if let Some(line) = iter.next() {
            fmt_line(line, f)?;

            for line in iter {
                f.write_char('\n')?;
                fmt_line(line, f)?;
            }
        }

        Ok(())
    }
}
