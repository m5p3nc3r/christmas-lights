#![cfg_attr(not(feature = "std"), no_std)]

pub use glam::f32::Vec2;
pub use glam::f32::Vec3;
pub mod shaders;

// TODO: Replace with crates.io colour library
#[derive(Clone, Copy, Default)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait RenderBuffer {
    fn size(&self) -> Vec2;
    fn buffer(&self) -> &[RGB8];
    fn buffer_mut(&mut self) -> &mut [RGB8];

    fn clear(&mut self) {
        for i in 0..self.buffer().len() {
            self.buffer_mut()[i] = RGB8::default();
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> RGB8 {
        let index = x + y * self.size().x as u32;
        self.buffer()[index as usize]
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: RGB8) {
        let index = x + y * self.size().x as u32;
        self.buffer_mut()[index as usize] = color;
    }
}

#[allow(non_snake_case)]
pub struct ShaderInput {
    pub iResolution: Vec3,
    pub iTime: f32,
    pub iTimeDelta: f32,
}

pub struct RenderEngine {}

impl RenderEngine {
    pub fn render(
        uniforms: &ShaderInput,
        f: fn(fragCoord: Vec2, &ShaderInput) -> RGB8,
        b: &mut impl RenderBuffer,
    ) {
        for x in 0..b.size().x as u32 {
            for y in 0..b.size().y as u32 {
                b.set_pixel(x, y, f(Vec2::new(x as f32, y as f32), uniforms));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
