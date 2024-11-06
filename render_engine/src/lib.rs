#![cfg_attr(not(feature = "std"), no_std)]

pub use glam::f32::Vec2;
pub use glam::f32::Vec3;
pub use glam::u32::UVec2;
use render::RenderType;
pub use renderbuffer::RenderBuffer;
use shaders::MainImageFn;
pub use shaders::Shader;
use shaders::ShaderInput;
pub mod shaders;
mod render;
mod renderbuffer;


// TODO: Replace with crates.io colour library
#[derive(Clone, Copy, Default)]
pub struct RGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Clone, Copy)]
pub enum RenderFunction{
    Shader(MainImageFn),
    Render(RenderType),
    None,
}


pub struct RenderEngine {
    shader: RenderFunction,
    transition_to_shader: RenderFunction,
    // TODO: Use Fixed for transition_duration
    transition_duration: f32,

    shader_engine: shaders::ShaderEngine,
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderEngine {
    pub fn new() -> Self {
        Self {
            shader: RenderFunction::None,
            transition_to_shader: RenderFunction::None,
            transition_duration: 0.0,

            shader_engine: shaders::ShaderEngine::new(),
        }
    }

    pub fn set_shader(&mut self, shader: Shader) {
        self.shader = RenderFunction::Shader(shader.to_main_image_fn());
    }

    pub fn set_transition_to_shader(&mut self, shader: Shader, duration: f32) {
        self.transition_to_shader = RenderFunction::Shader(shader.to_main_image_fn());
        self.transition_duration = duration;
    }

    pub fn render<const S: usize, const X: usize, const Y: usize>(&mut self, t: f32, dt: f32, b: &mut RenderBuffer<S, X, Y>) {
        // Calculate the uniforms
        let u = ShaderInput {
            iResolution: Vec3::new(b.size().x as f32, b.size().y as f32, 0.0),
            iTime: t,
            iTimeDelta: dt,
        };

        // TODO: Store this in the struct and reuse it
        let mut back_buffer = RenderBuffer::<S, X, Y>::new();

        match self.shader {
            RenderFunction::Shader(shader) => {
                self.shader_engine.render(t, dt, &mut back_buffer, &shader);
            }
            RenderFunction::Render(_render_type) => {
                //TODO: Implement me...
                //render_type.render(t, dt, &mut backBuffer);
            }
            RenderFunction::None => {}
        }

        let mut front_buffer = RenderBuffer::<S, X, Y>::new();
        match self.transition_to_shader {
            RenderFunction::Shader(shader) => {
                self.shader_engine.render(t, dt, &mut front_buffer, &shader);
            }
            RenderFunction::Render(_render_type) => {
            }
        _ => {}
        }


        self.transition_duration = 1.0;

        back_buffer.buffer().iter().zip(front_buffer.buffer().iter()).enumerate().for_each(|(index, (back, front))| {
            let new_color = RGB8 {
                r: (back.r as f32 * self.transition_duration + front.r as f32 * (1.0 - self.transition_duration)) as u8,
                g: (back.g as f32 * self.transition_duration + front.g as f32 * (1.0 - self.transition_duration)) as u8,
                b: (back.b as f32 * self.transition_duration + front.b as f32 * (1.0 - self.transition_duration)) as u8,
            };
          
            let x = index % X;
            let y = index / X;
            b.set_pixel(x as u32, y as u32, new_color);
        });

    }

}
