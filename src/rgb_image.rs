use std::slice::Chunks;

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
