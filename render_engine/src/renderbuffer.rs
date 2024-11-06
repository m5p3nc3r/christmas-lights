use glam::UVec2;
use crate::RGB8;

pub struct RenderBuffer<const S: usize, const X:usize, const Y:usize> {
    size: UVec2,
    buffer: [RGB8; S],
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
            buffer: [RGB8::default(); S],
        }
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn buffer(&self) -> &[RGB8] {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [RGB8] {
        &mut self.buffer
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer().len() {
            self.buffer_mut()[i] = RGB8::default();
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> RGB8 {
        let index = x + y * self.size().x;
        self.buffer()[index as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: RGB8) {
        let index = x + y * self.size().x;
        self.buffer_mut()[index as usize] = color;
    }
}
