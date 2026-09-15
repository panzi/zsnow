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

        let mut curr_fg = None;
        let mut curr_bg = None;

        for term_row_index in row..row + rows {
            let term_slice_index = term_row_index * self.size.width + column;
            let term_row = &mut self.data[term_slice_index..term_slice_index + columns];

            let line_index = term_row_index * 2;
            let line1 = image.get_line(line_index);
            let line2 = image.get_line(line_index + 1);

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
        if column >= self.size.width || row >= self.size.height {
            return;
        }

        let columns = image.size().width.div_ceil(2);
        let rows = image.size().height.div_ceil(3);

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

        let mut curr_bg: Option<Rgb> = None;

        for term_row_index in row..row + rows {
            let term_slice_index = term_row_index * self.size.width + column;
            let term_row = &mut self.data[term_slice_index..term_slice_index + columns];

            let line_index = term_row_index * 3;
            let line1 = image.get_line(line_index);
            let line2 = image.get_line(line_index + 1);
            let line3 = image.get_line(line_index + 2);

            for (cell_index, term_cell) in term_row.iter_mut().enumerate() {
                let x = cell_index * 2;

                let c11 = line1[x];
                let c12 = line1[x + 1];
                let c21 = line2[x];
                let c22 = line2[x + 1];
                let c31 = line3[x];
                let c32 = line3[x + 1];

                let colors = [
                    c11, c12,
                    c21, c22,
                    c31, c32,
                ];

                let palette = bin_means(&colors);
                let [color1, color2] = palette;

                if color1 == color2 {
                    term_cell.fg = color1;
                    term_cell.bg = curr_bg.unwrap_or(color1);
                    term_cell.c = FULL_BLOCK;
                } else {
                    let mut bits: u32 = 0;
                    for (bit, color) in colors.iter().cloned().enumerate() {
                        if get_clostest_color(color, &palette) == color1 {
                            bits |= 1 << bit;
                        }
                    }

                    term_cell.fg = color1;
                    term_cell.bg = color2;
                    term_cell.c = get_two_by_three_from_bits(bits);
                }

                curr_bg = Some(term_cell.bg);
            }
        }
    }

    pub fn draw_braille(&mut self, column: usize, row: usize, image: &RgbImage) {
        if column >= self.size.width || row >= self.size.height {
            return;
        }

        let columns = image.size().width.div_ceil(2);
        let rows = image.size().height.div_ceil(4);

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

        let mut curr_bg: Option<Rgb> = None;

        for term_row_index in row..row + rows {
            let term_slice_index = term_row_index * self.size.width + column;
            let term_row = &mut self.data[term_slice_index..term_slice_index + columns];

            let line_index = term_row_index * 4;
            let line1 = image.get_line(line_index);
            let line2 = image.get_line(line_index + 1);
            let line3 = image.get_line(line_index + 2);
            let line4 = image.get_line(line_index + 3);

            for (cell_index, term_cell) in term_row.iter_mut().enumerate() {
                let x = cell_index * 2;

                let c11 = line1[x];
                let c12 = line1[x + 1];
                let c21 = line2[x];
                let c22 = line2[x + 1];
                let c31 = line3[x];
                let c32 = line3[x + 1];
                let c41 = line4[x];
                let c42 = line4[x + 1];

                let colors = [
                    c11, c12,
                    c21, c22,
                    c31, c32,
                    c41, c42,
                ];

                let palette = bin_means(&colors);
                let [mut color1, mut color2] = palette;

                if color1 == color2 {
                    term_cell.fg = color1;
                    term_cell.bg = curr_bg.unwrap_or(color1);
                    term_cell.c = FULL_BLOCK;
                } else {
                    let mut bits: u32 = 0;

                    if let Some(curr_bg) = curr_bg {
                        if distance2(color1, curr_bg) < distance2(color2, curr_bg) {
                            std::mem::swap(&mut color1, &mut color2);
                        }
                    } else {
                        std::mem::swap(&mut color1, &mut color2);
                    }

                    for y in 0..4 {
                        for x in 0..2 {
                            let color = colors[(y * 2 + x) as usize];
                            if get_clostest_color(color, &palette) == color1 {
                                bits = set_pixel_in_braille(bits, x, y);
                            }
                        }
                    }

                    term_cell.fg = color1;
                    term_cell.bg = color2;
                    term_cell.c = get_braille_from_bits(bits);
                }

                curr_bg = Some(term_cell.bg);
            }
        }
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

        for (y, (row, prev_row)) in self.data.chunks(self.size.width).zip(prev_frame.data.chunks(prev_frame.size.width)).enumerate() {
            let y = y as u32;

            for (x, (c, prev_c)) in row.iter().zip(prev_row.iter()).enumerate() {
                let x = x as u32;

                if c != prev_c {
                    if x == 0 && prev_y + 1 == y {
                        termio.write_str("\n")?;
                    } else if prev_y == y && y != 0 {
                        let curr_x = prev_x + 1;
                        if curr_x == x {
                            // already at correct position
                        } else if curr_x < x {
                            termio.move_cursor_forward(x - curr_x)?;
                        } else {
                            termio.move_cursor_back(curr_x - x)?;
                        }
                    } else {
                        termio.move_cursor(y, x)?;
                    }

                    prev_x = x;
                    prev_y = y;

                    if Some(c.fg) != curr_fg {
                        termio.fg_rgb(c.fg)?;
                        //termio.fg_rgb(Rgb::from_u32(0xFF0000))?;
                        curr_fg = Some(c.fg);
                    }

                    if Some(c.bg) != curr_bg {
                        termio.bg_rgb(c.bg)?;
                        //termio.bg_rgb(Rgb::from_u32(0xFF7700))?;
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

        for row in self.data.chunks(self.size.width) {
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

const TWO_BY_THREE: [char; 64] = [
    ' ', '🬀', '🬁', '🬂', '🬃', '🬄', '🬅', '🬆', '🬇', '🬈', '🬉', '🬊', '🬋', '🬌', '🬍',
    '🬎', '🬏', '🬐', '🬑', '🬒', '🬓', '▌', '🬔', '🬕', '🬖', '🬗', '🬘', '🬙', '🬚', '🬛',
    '🬜', '🬝', '🬞', '🬟', '🬠', '🬡', '🬢', '🬣', '🬤', '🬥', '🬦', '🬧', '▐', '🬨', '🬩',
    '🬪', '🬫', '🬬', '🬭', '🬮', '🬯', '🬰', '🬱', '🬲', '🬳', '🬴', '🬵', '🬶', '🬷', '🬸',
    '🬹', '🬺', '🬻', '█',
];

#[inline]
pub fn get_two_by_three_from_bits(bits: u32) -> char {
    TWO_BY_THREE[bits as usize]
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

fn bin_means(colors: &[Rgb]) -> [Rgb; 2] {
    let mut centroids = [Rgb::from_u32(0x888888), Rgb::from_u32(0x888888)];
    let mut clusters = [Cluster::new(), Cluster::new()];
    let mut new_centroids = [Rgb::from_u32(0x888888), Rgb::from_u32(0x888888)];

    loop {
        // Create empty clusters
        for cluster in &mut clusters {
            cluster.clear();
        }

        // Assign each point to the nearest centroid
        for color in colors.iter().cloned() {
            let mut closest_index = 0;
            let mut min_distance = distance2(color, centroids[0]);

            let d = distance2(color, centroids[1]);
            if d < min_distance {
                min_distance = d;
                closest_index = 1;
            }

            clusters[closest_index].add(color);
        }

        // Recalculate centroids as the mean of each cluster
        for (cluster, new_centroid) in clusters.iter().zip(new_centroids.iter_mut()) {
            *new_centroid = cluster.to_centroid();
        }

        // Check for convergence
        if new_centroids == centroids {
            return new_centroids;
        }

        centroids = new_centroids;
    }
}

fn get_clostest_color(color: Rgb, palette: &[Rgb]) -> Rgb {
    let mut iter = palette.iter().cloned();

    if let Some(mut closest) = iter.next() {
        let mut closest_dist = distance2(color, closest);

        for other in iter {
            let dist = distance2(color, other);
            if dist < closest_dist {
                closest_dist = dist;
                closest = other;
            }
        }

        return closest;
    }

    color
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cluster {
    r: u32,
    g: u32,
    b: u32,
    count: u32,
}

impl Cluster {
    #[inline]
    fn new() -> Self {
        Self { r: 0, g: 0, b: 0, count: 0 }
    }

    #[inline]
    fn clear(&mut self) {
        self.r = 0;
        self.g = 0;
        self.b = 0;
        self.count = 0;
    }

    #[inline]
    fn add(&mut self, color: Rgb) {
        self.r += color.r as u32;
        self.g += color.g as u32;
        self.b += color.b as u32;
        self.count += 1;
    }

    #[inline]
    fn to_centroid(&self) -> Rgb {
        if self.count == 0 {
            return Rgb::from_u32(0x888888);
        }

        Rgb {
            r: (self.r / self.count) as u8,
            g: (self.g / self.count) as u8,
            b: (self.b / self.count) as u8,
        }
    }
}

fn kmeans(k: usize, colors: &[Rgb]) -> Vec<Rgb> {
    // https://en.wikipedia.org/wiki/K-means_clustering

    if k == 0 {
        return Vec::new();
    }

    // Initialize centroids
    let mut centroids = vec![Rgb::from_u32(0x888888); k];
    let mut clusters = vec![Cluster::new(); k];
    let mut new_centroids = Vec::with_capacity(k);

    loop {
        // Create empty clusters
        for cluster in &mut clusters {
            cluster.clear();
        }

        // Assign each point to the nearest centroid
        for color in colors.iter().cloned() {
            let mut closest_index = 0;
            let mut min_distance = distance2(color, centroids[0]);
            for j in 1..k {
                let d = distance2(color, centroids[j]);
                if d < min_distance {
                    min_distance = d;
                    closest_index = j;
                }
            }

            clusters[closest_index].add(color);
        }

        // Recalculate centroids as the mean of each cluster
        new_centroids.clear();
        for cluster in &clusters {
            new_centroids.push(cluster.to_centroid());
        }

        // Check for convergence
        if new_centroids == centroids {
            return new_centroids;
        }

        centroids.clear();
        centroids.extend_from_slice(&new_centroids);
    }
}

fn calc_centroid(cluster: &[Rgb]) -> Rgb {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;

    for color in cluster {
        r += color.r as u32;
        g += color.g as u32;
        b += color.b as u32;
    }

    let n = cluster.len() as u32;

    Rgb {
        r: (r / n) as u8,
        g: (g / n) as u8,
        b: (b / n) as u8,
    }
}

#[inline]
fn distance2(c1: Rgb, c2: Rgb) -> u32 {
    let dr = c1.r as i32 - c2.r as i32;
    let dg = c1.g as i32 - c2.g as i32;
    let db = c1.b as i32 - c2.b as i32;

    (dr * dr) as u32 + (dg * dg) as u32 + (db * db) as u32
}
