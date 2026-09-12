use std::slice::Chunks;

use crate::{color::Rgb, rect::Rect, size2d::Size2D};


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RgbImage {
    data: Box<[Rgb]>,
    size: Size2D,
}

impl RgbImage {
    #[inline]
    pub fn new(size: Size2D, fill: Rgb) -> Self {
        let data = vec![fill; size.width * size.height].into_boxed_slice();
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
        self.data[self.size.width * y + x]
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgb) {
        self.data[self.size.width * y + x] = color;
    }

    #[inline]
    pub fn clear(&mut self, color: Rgb) {
        self.data.fill(color);
    }

    #[inline]
    pub fn lines(&self) -> Chunks<'_, Rgb> {
        self.data.chunks(self.size.width)
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
