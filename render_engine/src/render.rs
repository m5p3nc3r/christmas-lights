use glam::UVec2;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::RenderBuffer;
use hex_color::HexColor;

#[derive(Clone, Copy)]
pub enum RenderType {
    Sparkle,
}

pub struct Renderers<const S: usize, const X: usize, const Y: usize> {
    sparkle: Sparkle,
}

impl<const S: usize, const X: usize, const Y: usize> Renderers<S, X, Y> {
    pub fn new() -> Self {
        Self {
            sparkle: Sparkle::new(),
        }
    }

    pub fn step(&mut self, rederer: RenderType) {
        match rederer {
            RenderType::Sparkle => <Sparkle as Render<S, X, Y>>::step(&mut self.sparkle),
        }
    }
    pub fn render(&self, renderer: RenderType, t: f32, dt: f32, buffer: &mut RenderBuffer<S, X, Y>) {
        match renderer {
            RenderType::Sparkle => self.sparkle.render(t, dt, buffer),
        }
    }
}



pub trait Render<const S: usize, const X: usize, const Y: usize> {
    fn step(&mut self);
    // TODO: Use Fixed for t and dt f32's
    fn render(&self, t: f32, dt: f32, buffer: &mut RenderBuffer<S, X, Y>);
}


#[derive(Clone, Copy)]
struct SparklePoint {
    pos: UVec2,
    color: HexColor,
    phase: u8,  // Changed to Fixed??
}

impl SparklePoint {
    fn random_pos(rng: &mut SmallRng) -> Self {
        Self {
            pos: UVec2::new(rng.gen_range(0..50), rng.gen_range(0..24)),
            color: HexColor::WHITE,
            phase: rng.gen_range(0..255),
        }
    }
}

const NUM_SPARKLE_POINTS: usize = 100;

struct Sparkle {
    points: [SparklePoint; NUM_SPARKLE_POINTS],
    rng: SmallRng,
}

impl Sparkle {
    fn new() -> Self {
        let mut rng = SmallRng::seed_from_u64(0);

        Self {
            points: core::array::from_fn(|_| SparklePoint::random_pos(&mut rng)),
            rng,
        }
    }
}

impl<const S:usize, const X: usize, const Y: usize> Render<S, X, Y> for Sparkle {
    fn step(&mut self) {
        for point in self.points.iter_mut() {
            if point.phase < 255 {
                point.phase +=1;
            } else {
                *point = SparklePoint::random_pos(&mut self.rng);
                point.phase = 0;
            }
        }
    }

    fn render(&self, _t: f32, _dt: f32, buffer: &mut RenderBuffer<S, X, Y>) {
        for point in self.points.iter() {
            let mut colour = point.color;
            colour.r = (colour.r as f32 * (point.phase as f32 / 255.0)) as u8;
            colour.g = (colour.g as f32 * (point.phase as f32 / 255.0)) as u8;
            colour.b = (colour.b as f32 * (point.phase as f32 / 255.0)) as u8;
            buffer.set_pixel(point.pos.x, point.pos.y, colour);
        }
    }
}