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

const WIDTH: usize = 50;
const HEIGHT: usize = 24;

pub struct RenderEngine {
    renderer: Renderer,
    transition: Option<Transition>,
    // TODO: Use Fixed for transition_duration

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
            transition: None,

            shader_engine: shaders::ShaderEngine::new(),
            render_engine: render::Renderers::new(),

            front_buffer: RenderBuffer::<{WIDTH * HEIGHT}, WIDTH, HEIGHT>::new(),
            back_buffer: RenderBuffer::<{WIDTH * HEIGHT}, WIDTH, HEIGHT>::new(),
        }
    }

    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;
    }

    pub fn tx_progress(&self) -> f32 {
        self.transition.as_ref().map(|t| t.progress()).unwrap_or(0.0)
    }

    pub fn set_transition_to_renderer(&mut self, renderer: Renderer, duration: f32) {
        self.transition = Some(Transition::new(renderer, duration));
    }

    pub fn render<const S: usize, const X: usize, const Y: usize>(&mut self, t: f32, dt: f32, b: &mut RenderBuffer<S, X, Y>) {

        if let Some(transition) = &mut self.transition {
            transition.step(dt);
            if transition.is_done() {
                self.renderer = transition.renderer;
                self.transition = None;
            }
        }

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
        if let Some(transition) = &mut self.transition {
            match transition.renderer {
                Renderer::Basic(r) => {
                    self.render_engine.step(r);
                    self.render_engine.render(r, t, dt, &mut self.front_buffer);
                }
                Renderer::Shader(s) => {
                    self.shader_engine.render(&s.to_main_image_fn(), t, dt, &mut self.front_buffer);
                }
                Renderer::None => {}
            }
        }

        let progress = self.transition.as_ref().map(|t| t.progress()).unwrap_or(0.0);

        self.back_buffer.buffer().iter().zip(self.front_buffer.buffer().iter()).enumerate().for_each(|(index, (back, front))| {
//            let new_color = back.scale(1.0 - progress) + front.scale(progress);
            let new_color = back.scale(1.0 - progress).checked_add(front.scale(progress)).unwrap_or(HexColor::WHITE);

            b.set_pixel((index % X) as u32, (index / X) as u32, new_color);
        });

    }

}


struct Transition {
    pub renderer: Renderer,
    duration: f32,
    current: f32,
}

impl Transition {
    fn new(renderer: Renderer, duration: f32) -> Self {
        Self {
            renderer,
            duration,
            current: 0.0,
        }
    }

    fn step(&mut self, dt: f32) {
        self.current += dt;
    }

    fn is_done(&self) -> bool {
        self.current >= self.duration
    }

    fn progress(&self) -> f32 {
        if self.current >= self.duration || self.duration == 0.0 {
            return 0.0;
        } else {
            self.current / self.duration
        }
    }
}