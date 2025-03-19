use crate::renderbuffer::Blend;
use crate::{UVec2, Vec2};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use az::Cast;

use crate::fixedcolor::FixedColor;
use crate::RenderBuffer;

#[derive(Clone, Copy, PartialEq)]
pub enum RenderType {
    Sparkle,
    Snow,
    Rainbow,
}

pub struct Renderers<const S: usize, const X: usize, const Y: usize> {
    sparkle: Sparkle<X, Y>,
    snow: Snow<X, Y>,
    rainbow: Rainbow<X, Y>,
}

impl<const S: usize, const X: usize, const Y: usize> Renderers<S, X, Y> {
    pub fn new() -> Self {
        Self {
            sparkle: Sparkle::new(),
            snow: Snow::new(),
            rainbow: Rainbow::new(),
        }
    }

    pub fn step(&mut self, renderer: RenderType) {
        match renderer {
            RenderType::Sparkle => <Sparkle<X, Y> as Render<S, X, Y>>::step(&mut self.sparkle),
            RenderType::Snow => <Snow<X, Y> as Render<S, X, Y>>::step(&mut self.snow),
            RenderType::Rainbow => <Rainbow<X, Y> as Render<S, X, Y>>::step(&mut self.rainbow),
        }
    }

    pub fn render(&self, renderer: RenderType, t: f32, dt: f32, buffer: &mut RenderBuffer<S, X, Y>, blend: Blend) {
        match renderer {
            RenderType::Sparkle => self.sparkle.render(t, dt, buffer, blend),
            RenderType::Snow => self.snow.render(t, dt, buffer, blend),
            RenderType::Rainbow => self.rainbow.render(t, dt, buffer, blend),
        }
    }
}



pub trait Render<const S: usize, const X: usize, const Y: usize> {
    fn step(&mut self);
    fn render(&self, t: f32, dt: f32, buffer: &mut RenderBuffer<S, X, Y>, blend: Blend);
}


#[derive(Clone, Copy)]
struct SparklePoint {
    pos: UVec2,
    color: FixedColor,
    phase: f32,
    speed: f32,
}

impl SparklePoint {
    fn random_pos(rng: &mut SmallRng, x_max: u32, y_max: u32) -> Self {

        let phase = rng.gen();
        let speed = rng.gen_range(0.005..0.05);

        Self {
            pos: UVec2::new(rng.gen_range(0..x_max), rng.gen_range(0..y_max)),
            color: FixedColor::WHITE,
            phase,
            speed,
        }
    }
}

const NUM_SPARKLE_POINTS: usize = 20;

struct Sparkle<const X: usize, const Y: usize> {
    points: [SparklePoint; NUM_SPARKLE_POINTS],
    rng: SmallRng,
}

impl <const X: usize, const Y: usize> Sparkle<X, Y> {
    fn new() -> Self {
        let mut rng = SmallRng::seed_from_u64(0);

        Self {
            points: core::array::from_fn(|_| SparklePoint::random_pos(&mut rng, X as u32, Y as u32)),
            rng,
        }
    }
}

impl<const S:usize, const X: usize, const Y: usize> Render<S, X, Y> for Sparkle<X, Y> {
    fn step(&mut self) {
        for point in self.points.iter_mut() {
            let phase = point.phase + point.speed;
            if phase >= 1.0 {
                point.phase = phase;
            } else {
                *point = SparklePoint::random_pos(&mut self.rng, X as u32, Y as u32);
                point.phase = 0.0;
            }
        }
    }

    fn render(&self, _t: f32, _dt: f32, buffer: &mut RenderBuffer<S, X, Y>, _blend: Blend) {
        for point in self.points.iter() {
            let colour = point.color.scale(point.phase.cast());
            buffer.safe_set_pixel(point.pos.x, point.pos.y, colour);
        }
    }
}
// -----

const NUM_SNOWFLAKES: usize = 30;
const MAX_SNOWFLAKE_SPEED: f32 = 0.5;
const MIN_SNOWFLAKE_SPEED: f32 = 0.1;
struct SnowFlake {
    pos: Vec2,
    speed: f32,
    color: FixedColor,
}

impl SnowFlake {
    fn new_random(rng: &mut SmallRng, x_max: usize, y_max: usize) -> Self {
        let min = MIN_SNOWFLAKE_SPEED;
        let max = MAX_SNOWFLAKE_SPEED;

        let speed = rng.gen_range(min..max);
        let scale = (speed - min) / (max - min);
        let color = FixedColor::WHITE.scale(scale);


        Self { 
            pos:  Vec2 {
                x: rng.gen_range(0..x_max) as f32,
                y: rng.gen_range(0..y_max) as f32,
            },
            speed,
            color,
        }
    }

    fn new_randon_top(&mut self, rng: &mut SmallRng, x: usize) {
        self.pos = Vec2 {
            x: rng.gen_range(0..x) as f32,
            y: 0.0,
        };
    }
}

struct Snow<const X: usize, const Y: usize> {
    // Would like to make NUM_SNOWFLAKES something like X * Y / 6
    snowflakes: [SnowFlake; NUM_SNOWFLAKES],
    rng: SmallRng,
}

impl<const X: usize, const Y: usize> Snow<X, Y> {
    fn new() -> Self {
        let mut rng = SmallRng::seed_from_u64(0);

        Self {
            snowflakes: core::array::from_fn(|_| SnowFlake::new_random(&mut rng, X, Y)),
            rng,
        }
    }
}

impl<const S: usize, const X: usize, const Y: usize> Render<S, X, Y> for Snow<X, Y> {
    fn step(&mut self) {
        for snowflake in self.snowflakes.iter_mut() {
            snowflake.pos.y += snowflake.speed;
            if snowflake.pos.y > Y as f32 {
                snowflake.new_randon_top(&mut self.rng, X);
            }
        }
    }

    fn render(&self, _t: f32, _dt: f32, buffer: &mut RenderBuffer<S, X, Y>, blend: Blend) {
        for snowflake in self.snowflakes.iter() {

            let one = 1.0;
            let phase = snowflake.pos.y % 1.0;

            let x: u32 = snowflake.pos.x.cast();
            let y: u32 = snowflake.pos.y.cast();

            buffer.safe_set_max_rgb(x, y, snowflake.color.scale(one - phase), blend);
            if phase >0.0 {
                buffer.safe_set_max_rgb(x, y+1, snowflake.color.scale(phase), blend);
            }
        }
    }
}

struct Rainbow<const X: usize, const Y: usize> {
    phase: f32,
}

impl<const X: usize, const Y: usize> Rainbow<X, Y> {
    fn new() -> Self {
        Self {
            phase: 0.0,
        }
    }
}


impl<const S: usize, const X: usize, const Y: usize>  Render<S, X, Y> for Rainbow<X, Y> {
    fn step(&mut self) {
        self.phase += 0.05;
    }

    fn render(&self, _t: f32, _dt: f32, buffer: &mut RenderBuffer<S, X, Y>, _blend: Blend) {
        for x in 0..X {
            let offset = x as f32 / X as f32;
            for y in 0..Y {

                let r = libm::sinf((self.phase + offset) * 2.0) * 0.5 + 0.5;
                let g = libm::sinf((self.phase + offset) * 0.7) * 0.5 + 0.5;
                let b = libm::sinf((self.phase + offset) * 1.3) * 0.5 + 0.5;
    
                let c = FixedColor::rgb(r, g, b);
                buffer.safe_set_pixel(x as u32, y as u32, c);
            }
        }
    }
}
