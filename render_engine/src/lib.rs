#![cfg_attr(not(feature = "std"), no_std)]

pub use glam::f32::Vec2;
pub use glam::f32::Vec3;
pub use glam::u32::UVec2;
mod octograms;
pub mod shaders;
mod render;

// TODO: Replace with crates.io colour library
#[derive(Clone, Copy, Default)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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


#[allow(non_snake_case)]
pub struct ShaderInput {
    pub iResolution: Vec3,
    pub iTime: f32,
    pub iTimeDelta: f32,
}

type MainImageFn = fn(Vec2, &ShaderInput) -> RGB8;

#[derive(Clone, Copy)]
pub enum ShaderFunction {
    MainImage(MainImageFn),
    None,
}


pub enum Shader {
    Rainbow,
    HypnoticRectangles,
    Octograms,
}

impl Shader {
    fn to_main_image_fn(&self) -> MainImageFn {
        match self {
            Shader::Rainbow => shaders::rainbow,
            Shader::HypnoticRectangles => shaders::hypnotic_rectangles,
            Shader::Octograms => octograms::octograms,
        }
    }
}

pub struct ShaderEngine {
    shader: ShaderFunction,
    transition_to_shader: ShaderFunction,
    transition_duration: f32,
}

impl Default for ShaderEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ShaderEngine {
    pub fn new() -> Self {
        Self {
            shader: ShaderFunction::None,
            transition_to_shader: ShaderFunction::None,
            transition_duration: 0.0,
        }
    }

    pub fn set_shader(&mut self, shader: Shader) {
        self.shader = ShaderFunction::MainImage(shader.to_main_image_fn());
    }

    pub fn set_transition_to_shader(&mut self, shader: Shader, duration: f32) {
        self.transition_to_shader = ShaderFunction::MainImage(shader.to_main_image_fn());
        self.transition_duration = duration;
    }

    pub fn render<const S: usize, const X: usize, const Y: usize>(&mut self, t: f32, dt: f32, b: &mut RenderBuffer<S, X, Y>) {
        // Calculate the uniforms
        let u = ShaderInput {
            iResolution: Vec3::new(b.size().x as f32, b.size().y as f32, 0.0),
            iTime: t,
            iTimeDelta: dt,
        };

        //let tmp = RenderBuffer::<S, X, Y>::new();

        if let ShaderFunction::MainImage(shader) = &self.shader {
            Self::blend(&u, b, shader, 1.0);
        }
        if let ShaderFunction::MainImage(shader) = &self.transition_to_shader {
            Self::blend(&u, b, shader, 1.0 - self.transition_duration);

            // TODO: Calculate duration based on frame rate
            self.transition_duration -= 0.04;
            // Replace shader with transition_to_shader if transition_duration is 0
            if self.transition_duration <= 0.0 {
                self.shader = self.transition_to_shader;
                self.transition_to_shader = ShaderFunction::None;
            }
        }
    }

    pub fn blend<const S:usize, const X:usize, const Y:usize>(
        uniforms: &ShaderInput,
        b: &mut RenderBuffer<S, X, Y>,
        f: &MainImageFn,
        part_b: f32,
    ) {
        let part_a = 1.0 - part_b;

        for x in 0..b.size().x {
            for y in 0..b.size().y {
                let color = f(Vec2::new(x as f32, y as f32), uniforms);
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
