#![no_std]

use renderbuffer::Blend;
pub use vec::UVec2;
pub use render::RenderType;
pub use renderbuffer::RenderBuffer;
//pub use shaders::Shader;
//pub mod shaders;
mod render;
mod renderbuffer;
mod transition;
pub mod fixedcolor;
mod vec;

use transition::Transition;

pub type Fixed = fixed::FixedI32<fixed::types::extra::U24>;

#[derive(Clone, Copy, PartialEq)]
pub enum Renderer {
    Basic(render::RenderType),
//    Shader(shaders::Shader),
    None
}

pub struct RenderEngine<const S: usize, const X: usize, const Y: usize> {
    renderer: Renderer,
    transition: Option<Transition<Fixed>>,
    render_engine: render::Renderers<S, X, Y>,
}

impl<const S: usize, const X: usize, const Y: usize> Default for RenderEngine<S, X, Y> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const S: usize, const X: usize, const Y: usize> RenderEngine<S, X, Y> {
    pub fn new() -> Self {
        Self {
            renderer: Renderer::None,
            transition: None,

            render_engine: render::Renderers::new(),
        }
    }

    pub fn set_renderer(&mut self, renderer: Renderer) {
        self.renderer = renderer;
    }

    pub fn get_renderer(&self) -> Renderer {
        self.renderer
    }

    pub fn tx_progress(&self) -> Fixed {
        self.transition.as_ref().map(|t| t.progress()).unwrap_or(Fixed::ZERO)
    }

    pub fn set_transition_to_renderer(&mut self, renderer: Renderer, duration: Fixed) {
        self.transition = Some(Transition::new(renderer, duration));
    }

    pub fn render(&mut self, t: Fixed, dt: Fixed, b: &mut RenderBuffer<S, X, Y>) {
        if let Some(transition) = &mut self.transition {
            transition.step(dt);
            if transition.is_done() {
                self.renderer = transition.renderer;
                self.transition = None;
            }
        }

        match self.renderer {
            Renderer::Basic(r) => {
                // Only clear the buffer if there is something to render
                b.clear();
                self.render_engine.step(r);
                self.render_engine.render(r, t, dt, b, Blend::Dest);
            }
            Renderer::None => {}
        }

        // self.front_buffer.clear();
        // if let Some(transition) = &mut self.transition {
        //     match transition.renderer {
        //         Renderer::Basic(r) => {
        //             self.render_engine.step(r);
        //             self.render_engine.render(r, t, dt, &mut self.front_buffer);
        //         }
        //         Renderer::None => {}
        //     }
        // }

        // let progress = self.transition.as_ref().map(|t| t.progress()).unwrap_or(Fixed::ZERO);

        // self.back_buffer.buffer().iter().zip(self.front_buffer.buffer().iter()).enumerate().for_each(|(index, (back, front))| {
        //     let x = Fixed::ONE - progress;
        //     let y = progress;
        //     let new_color = back.scale(x).saturating_add(front.scale(y));

        //     b.safe_set_pixel((index % X) as u32, (index / X) as u32, new_color);
        // });

    }
}