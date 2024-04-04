#![cfg_attr(not(feature = "std"), no_std)]

pub use glam::f32::Vec2;
pub use glam::f32::Vec3;
pub mod shaders;

use shaders::ShaderPass;
use shaders::{HypnoticRectanges, Rainbow, Snow};

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

pub enum Shader {
    Rainbow,
    HypnoticRectangles,
    Snow,
}

struct Shaders {
    rainbow: Rainbow,
    hypnotic_rectangles: HypnoticRectanges,
    snow: Snow,
}

impl Default for Shaders {
    fn default() -> Self {
        Self {
            rainbow: Rainbow {},
            hypnotic_rectangles: HypnoticRectanges {},
            snow: Snow::default(),
        }
    }
}

impl Shaders {
    fn get_shader(&mut self, shader: &Shader) -> &mut dyn ShaderPass {
        match shader {
            Shader::Rainbow => &mut self.rainbow,
            Shader::HypnoticRectangles => &mut self.hypnotic_rectangles,
            Shader::Snow => &mut self.snow,
        }
    }
}

#[derive(Default)]
pub struct RenderEngine {
    shaders: Shaders,
    shader: Option<Shader>,
    transition_to_shader: Option<Shader>,
    transition_duration: f32,
}

impl RenderEngine {
    pub fn new() -> Self {
        Self {
            shaders: Shaders::default(),
            shader: None,
            transition_to_shader: None,
            transition_duration: 0.0,
        }
    }

    pub fn set_shader(&mut self, shader: Shader) {
        self.shader = Some(shader);
    }

    pub fn set_transition_to_shader(&mut self, shader: Shader, duration: f32) {
        self.transition_to_shader = Some(shader);
        self.transition_duration = duration;
    }

    pub fn render(&mut self, u: &ShaderInput, b: &mut impl RenderBuffer) {
        if let Some(shader) = &self.shader {
            let s = self.shaders.get_shader(shader);
            s.step();
            for x in 0..b.size().x as u32 {
                for y in 0..b.size().y as u32 {
                    b.set_pixel(x, y, s.mainImage(Vec2::new(x as f32, y as f32), u));
                }
            }
        }
        if let Some(transition_to_shader) = &self.transition_to_shader {
            let s: &mut dyn ShaderPass = self.shaders.get_shader(transition_to_shader);

            Self::blend(u, b, s, 1.0 - self.transition_duration);
            self.transition_duration -= 0.001;
            if self.transition_duration <= 0.0 {
                self.shader = self.transition_to_shader.take();
                self.transition_to_shader = None;
            }
        }
    }

    pub fn blend(
        uniforms: &ShaderInput,
        b: &mut impl RenderBuffer,
        s: &mut dyn ShaderPass,
        part_b: f32,
    ) {
        let part_a = 1.0 - part_b;

        for x in 0..b.size().x as u32 {
            for y in 0..b.size().y as u32 {
                let color = s.mainImage(Vec2::new(x as f32, y as f32), uniforms);
                let old_color = b.get_pixel(x, y);
                let new_color = RGB8 {
                    r: (old_color.r as f32 * part_a + color.r as f32 * part_b) as u8,
                    g: (old_color.g as f32 * part_a + color.g as f32 * part_b) as u8,
                    b: (old_color.b as f32 * part_a + color.b as f32 * part_b) as u8,
                };
                b.set_pixel(x, y, new_color);
            }
        }
    }
}
