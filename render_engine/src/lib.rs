#![cfg_attr(not(feature = "std"), no_std)]

pub use glam::f32::Vec2;
pub use glam::f32::Vec3;
pub mod shaders;

use shaders::ShaderPass;

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

#[derive(Default)]
pub struct RenderEngine<'a> {
    shader: Option<&'a dyn ShaderPass>,
    transition_to_shader: Option<&'a dyn ShaderPass>,
    transition_duration: f32,
}

impl<'a> RenderEngine<'a> {
    pub fn new() -> Self {
        Self {
            shader: None,
            transition_to_shader: None,
            transition_duration: 0.0,
        }
    }

    pub fn set_shader(&mut self, shader: &'a dyn ShaderPass) {
        self.shader = Some(shader);
    }

    pub fn set_transition_to_shader(&mut self, shader: &'a dyn ShaderPass, duration: f32) {
        self.transition_to_shader = Some(shader);
        self.transition_duration = duration;
    }

    pub fn render(&mut self, uniforms: &ShaderInput, b: &mut impl RenderBuffer) {
        if let Some(shader) = self.shader {
            for x in 0..b.size().x as u32 {
                for y in 0..b.size().y as u32 {
                    b.set_pixel(
                        x,
                        y,
                        shader.mainImage(Vec2::new(x as f32, y as f32), uniforms),
                    );
                }
            }
        }
        if let Some(transition_to_shader) = self.transition_to_shader {
            self.blend(
                uniforms,
                b,
                transition_to_shader,
                1.0 - self.transition_duration,
            );
            self.transition_duration -= 0.01;
            if (self.transition_duration <= 0.0) {
                self.shader = self.transition_to_shader;
                self.transition_to_shader = None;
            }
        }
    }

    pub fn blend(
        &self,
        uniforms: &ShaderInput,
        b: &mut impl RenderBuffer,
        s: &dyn ShaderPass,
        fraction: f32,
    ) {
        if let Some(shader) = self.shader {
            for x in 0..b.size().x as u32 {
                for y in 0..b.size().y as u32 {
                    let color = s.mainImage(Vec2::new(x as f32, y as f32), uniforms);
                    let old_color = b.get_pixel(x, y);
                    let new_color = RGB8 {
                        r: (old_color.r as f32 * 1.0 - fraction + color.r as f32 * fraction) as u8,
                        g: (old_color.g as f32 * 1.0 - fraction + color.g as f32 * fraction) as u8,
                        b: (old_color.b as f32 * 1.0 - fraction + color.b as f32 * fraction) as u8,
                    };
                    b.set_pixel(x, y, new_color);
                }
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
