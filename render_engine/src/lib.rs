#![cfg_attr(not(feature = "std"), no_std)]

pub use glam::f32::Vec2;
pub use glam::f32::Vec3;
pub use glam::u32::UVec2;
use hex_color::HexColor;
pub use render::RenderType;
pub use renderbuffer::RenderBuffer;
pub use shaders::Shader;
pub mod shaders;
mod render;
mod renderbuffer;


#[derive(Clone, Copy)]
pub enum Renderer {
    Basic(render::RenderType),
    Shader(shaders::Shader),
    None
}


// TODO: Replace with crates.io colour library
// #[derive(Clone, Copy, Default)]
// pub struct RGB8 {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }

const WIDTH: usize = 50;
const HEIGHT: usize = 24;

pub struct RenderEngine {
    renderer: Renderer,
    transition_to_renderer: Renderer,
    // TODO: Use Fixed for transition_duration
    transition_duration: f32,

    shader_engine: shaders::ShaderEngine,
    // TODO: Place constraints in RenderEngine struct
    render_engine: render::Renderers<{WIDTH * HEIGHT}, WIDTH, HEIGHT>,
    front_buffer: RenderBuffer<{WIDTH * HEIGHT}, WIDTH, HEIGHT>,
    back_buffer: RenderBuffer<{WIDTH * HEIGHT}, WIDTH, HEIGHT>,
}

impl Default for RenderEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderEngine {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::None,
            transition_to_renderer: Renderer::None,
            transition_duration: 0.0,

            shader_engine: shaders::ShaderEngine::new(),
            render_engine: render::Renderers::new(),

            front_buffer: RenderBuffer::<{WIDTH * HEIGHT}, WIDTH, HEIGHT>::new(),
            back_buffer: RenderBuffer::<{WIDTH * HEIGHT}, WIDTH, HEIGHT>::new(),
        }
    }

    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;
    }

    pub fn set_transition_to_renderer(&mut self, renderer: Renderer, duration: f32) {
        self.transition_to_renderer = renderer;
        self.transition_duration = duration;
    }

    pub fn render<const S: usize, const X: usize, const Y: usize>(&mut self, t: f32, dt: f32, b: &mut RenderBuffer<S, X, Y>) {
        self.back_buffer.clear();
        match self.renderer {
            Renderer::Basic(r) => {
                self.render_engine.step(r);
                self.render_engine.render(r, t, dt, &mut self.back_buffer);
            }
            Renderer::Shader(s) => {
                self.shader_engine.render(&s.to_main_image_fn(), t, dt, &mut self.back_buffer);
            }
            Renderer::None => {}
        }

        self.front_buffer.clear();
        match self.transition_to_renderer {
            Renderer::Basic(r) => {
                self.render_engine.step(r);
                self.render_engine.render(r, t, dt, &mut self.front_buffer);
            }
            Renderer::Shader(s) => {
                self.shader_engine.render(&s.to_main_image_fn(), t, dt, &mut self.front_buffer);
            }
            Renderer::None => {}
        }

        self.transition_duration = 1.0;



        self.back_buffer.buffer().iter().zip(self.front_buffer.buffer().iter()).enumerate().for_each(|(index, (back, front))| {
            let new_color = HexColor::rgb(
                (back.r as f32 * self.transition_duration + front.r as f32 * (1.0 - self.transition_duration)) as u8,
                (back.g as f32 * self.transition_duration + front.g as f32 * (1.0 - self.transition_duration)) as u8,
                (back.b as f32 * self.transition_duration + front.b as f32 * (1.0 - self.transition_duration)) as u8,
            );
          
            let x = index % X;
            let y = index / X;
            b.set_pixel(x as u32, y as u32, new_color);
        });

    }

}
