use glam::UVec2;

use crate::fixedcolor::FixedColor;
pub struct RenderBuffer<const S: usize, const X:usize, const Y:usize> {
    size: UVec2,
    buffer: [FixedColor; S],
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
            buffer: [FixedColor::default(); S],
        }
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn buffer(&self) -> &[FixedColor] {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut [FixedColor] {
        &mut self.buffer
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer().len() {
            self.buffer_mut()[i] = FixedColor::default();
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> FixedColor {
        let index = x + y * self.size().x;
        self.buffer()[index as usize]
    }

    pub fn safe_set_pixel(&mut self, x: u32, y: u32, color: FixedColor) {
        if x < X as u32 && y < Y as u32 {
            let index = x + y * self.size().x;
            self.buffer_mut()[index as usize] = color;
        }
    }

    pub fn safe_set_max_rgb(&mut self, x: u32, y: u32, color: FixedColor) {
        if x < X as u32 && y < Y as u32 {
            let current = self.get_pixel(x, y);
            let new_color = FixedColor::rgb (
                color.r.max(current.r),
                color.g.max(current.g),
                color.b.max(current.b),
            );
                let index = x + y * self.size().x;
            self.buffer_mut()[index as usize] = new_color;
        }
    }
}
