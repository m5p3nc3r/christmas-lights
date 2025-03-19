
use crate::{fixedcolor, UVec2};
use crate::fixedcolor::FixedColor;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_with::serde_as;

// Use of cfg_eval explained [here](https://docs.rs/serde_with/latest/serde_with/guide/serde_as/index.html#gating-serde_as-on-features)
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as, derive(Serialize, Deserialize))]
pub struct RenderBuffer<const S: usize, const X: usize, const Y: usize> {
    size: UVec2,
//    #[cfg_attr(feature = "serde", serde_as(as = "[_; S]"))]
    #[cfg_attr(feature = "serde", serde_as(as = "[_; S]"))]
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
            size: UVec2::new(X as u32, Y as u32),
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
        self.clear_to_color(FixedColor::default());
    }

    pub fn clear_to_color(&mut self, color: FixedColor) {
        for p in self.buffer_mut().iter_mut() {
            *p = color;
        }
    }

    #[inline(always)]
    fn index(&self, x: u32, y: u32) -> usize {
        (x + y * self.size().x) as usize
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> FixedColor {
        self.buffer()[self.index(x, y)]
    }

    pub fn safe_set_pixel(&mut self, x: u32, y: u32, color: FixedColor) {
        if x < X as u32 && y < Y as u32 {
            let index = self.index(x, y);
            self.buffer_mut()[index] = color;
        }
    }

    pub fn safe_set_max_rgb(&mut self, x: u32, y: u32, color: FixedColor, _blend: Blend) {
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

    pub fn blend_rgb(&mut self, x: u32, y: u32, color: FixedColor, _phase: f32, b: Blend)  {
        let src = self.get_pixel(x, y);
        let new_color = b.blend(src, color);
        self.safe_set_pixel(x, y, new_color);
    }
}


#[derive(Clone, Copy)]
pub enum Blend {
    Src,
    Dest,
    Merge(f32),
}

impl Blend {
    pub fn blend(&self, src: FixedColor, dest: FixedColor) -> FixedColor {
        match self {
            Blend::Src => src,
            Blend::Dest => dest,
            Blend::Merge(fixed) => blend_merge(src, dest, *fixed),
        }
    }
}

pub fn blend_merge(src: FixedColor, dest: FixedColor, phase: fixedcolor::T) -> FixedColor {
    let a = src.scale(1.0 - phase);
    let b = dest.scale(phase);

    a.saturating_add(b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "serde")]
    mod serde_tests {
        #[test]
        fn test_stream() {
            use super::*;
            use ciborium::{de::from_reader, ser::into_writer};
        
            type Buffer = RenderBuffer::<{4 * 4}, 4, 4>;

            let mut buffer = Buffer::new();
            let mut store = [0u8; 1024];
            buffer.safe_set_pixel(1, 1, FixedColor::WHITE);

            let _ = into_writer(&buffer, &mut store[..]).unwrap();

            let b2 : Buffer = from_reader(&store[..]).unwrap();

            assert_eq!(buffer.get_pixel(1, 1), b2.get_pixel(1, 1));
        }
    }
}