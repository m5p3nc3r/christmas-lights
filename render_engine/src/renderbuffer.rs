use glam::UVec2;
use hex_color::HexColor;

pub struct RenderBuffer<const S: usize, const X:usize, const Y:usize> {
    size: UVec2,
    buffer: [HexColor; S],
}

impl<const S: usize, const X:usize, const Y:usize> Default for RenderBuffer<S, X, Y> {
    fn default() -> Self {
        Self::new()
    }
}


impl<const S: usize, const X:usize, const Y:usize> RenderBuffer<S, X, Y> {
    pub fn new() -> Self {
        assert!(X * Y == S);
        Self {
            size: glam::u32::UVec2::new(X as u32, Y as u32),
            buffer: [HexColor::default(); S],
        }
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn buffer(&self) -> &[HexColor] {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [HexColor] {
        &mut self.buffer
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer().len() {
            self.buffer_mut()[i] = HexColor::default();
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> HexColor {
        let index = x + y * self.size().x;
        self.buffer()[index as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: HexColor) {
        let index = x + y * self.size().x;
        self.buffer_mut()[index as usize] = color;
    }
}
