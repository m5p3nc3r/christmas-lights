#![no_std]

pub use vec::UVec2;
pub use render::RenderType;
pub use renderbuffer::RenderBuffer;
//pub use shaders::Shader;
//pub mod shaders;
mod render;
mod renderbuffer;
mod transition;
mod fixedcolor;
mod vec;

use fixed;
use transition::Transition;

pub type Fixed = fixed::FixedI32<fixed::types::extra::U24>;

#[derive(Clone, Copy)]
pub enum Renderer {
    Basic(render::RenderType),
//    Shader(shaders::Shader),
    None
}

const WIDTH: usize = 50;
const HEIGHT: usize = 24;

pub struct RenderEngine {
    renderer: Renderer,
    transition: Option<Transition<Fixed>>,
    // TODO: Use Fixed for transition_duration

//    shader_engine: shaders::ShaderEngine,
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

//            shader_engine: shaders::ShaderEngine::new(),
            render_engine: render::Renderers::new(),

            front_buffer: RenderBuffer::<{WIDTH * HEIGHT}, WIDTH, HEIGHT>::new(),
            back_buffer: RenderBuffer::<{WIDTH * HEIGHT}, WIDTH, HEIGHT>::new(),
        }
    }

    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;
    }

    pub fn tx_progress(&self) -> Fixed {
        self.transition.as_ref().map(|t| t.progress()).unwrap_or(Fixed::ZERO)
    }

    pub fn set_transition_to_renderer(&mut self, renderer: Renderer, duration: Fixed) {
        self.transition = Some(Transition::new(renderer, duration));
    }

    pub fn render<const S: usize, const X: usize, const Y: usize>(&mut self, t: Fixed, dt: Fixed, b: &mut RenderBuffer<S, X, Y>) {
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
            // Renderer::Shader(s) => {
            //     self.shader_engine.render(&s.to_main_image_fn(), t, dt, &mut self.back_buffer);
            // }
            Renderer::None => {}
        }

        self.front_buffer.clear();
        if let Some(transition) = &mut self.transition {
            match transition.renderer {
                Renderer::Basic(r) => {
                    self.render_engine.step(r);
                    self.render_engine.render(r, t, dt, &mut self.front_buffer);
                }
                // Renderer::Shader(s) => {
                //     self.shader_engine.render(&s.to_main_image_fn(), t, dt, &mut self.front_buffer);
                // }
                Renderer::None => {}
            }
        }

        let progress = self.transition.as_ref().map(|t| t.progress()).unwrap_or(Fixed::ZERO);

        self.back_buffer.buffer().iter().zip(self.front_buffer.buffer().iter()).enumerate().for_each(|(index, (back, front))| {
//            let new_color = back.scale(1.0 - progress) + front.scale(progress);
            let x = Fixed::ONE - progress;
            let y = progress;
            let new_color = back.scale(x).saturating_add(front.scale(y));

            b.safe_set_pixel((index % X) as u32, (index / X) as u32, new_color);
        });

    }

}