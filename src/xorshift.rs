#[derive(Debug, Copy, Clone)]
pub struct XOrShift32 {
    state: u32,
}

const F32_SCALE: f32 = 1.0 / u32::MAX as f32;

impl XOrShift32 {
    /// Must be initialized with non-zero.
    #[inline]
    pub fn new(state: u32) -> Self {
        Self { state }
    }

    pub fn next(&mut self) -> u32 {
        let mut state = self.state;
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        self.state = state;
        state
    }

    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        self.next() as f32 * F32_SCALE
    }

    #[inline]
    pub fn state(&self) -> u32 {
        self.state
    }
}
