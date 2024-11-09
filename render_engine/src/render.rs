use glam::{UVec2, Vec2};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::RenderBuffer;
use hex_color::HexColor;

#[derive(Clone, Copy)]
pub enum RenderType {
    Sparkle,
    Snow,
}

pub struct Renderers<const S: usize, const X: usize, const Y: usize> {
    sparkle: Sparkle,
    snow: Snow,
}

impl<const S: usize, const X: usize, const Y: usize> Renderers<S, X, Y> {
    pub fn new() -> Self {
        Self {
            sparkle: Sparkle::new(),
            snow: Snow::new(),
        }
    }

    pub fn step(&mut self, renderer: RenderType) {
        match renderer {
            RenderType::Sparkle => <Sparkle as Render<S, X, Y>>::step(&mut self.sparkle),
            RenderType::Snow => <Snow as Render<S, X, Y>>::step(&mut self.snow),

        }
    }
    pub fn render(&self, renderer: RenderType, t: f32, dt: f32, buffer: &mut RenderBuffer<S, X, Y>) {
        match renderer {
            RenderType::Sparkle => self.sparkle.render(t, dt, buffer),
            RenderType::Snow => self.snow.render(t, dt, buffer),

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

const NUM_SPARKLE_POINTS: usize = 200;

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
            let colour = point.color.scale(point.phase as f32 / 255.0);
            buffer.safe_set_pixel(point.pos.x, point.pos.y, colour);
        }
    }
}
// -----

const NUM_SNOWFLAKES: usize = 200;
const MAX_SNOWFLAKE_SPEED: f32 = 0.5;
const MIN_SNOWFLAKE_SPEED: f32 = 0.1;
struct SnowFlake {
    pos: Vec2,
    speed: f32,
    color: HexColor,
}

impl SnowFlake {
    fn new_random(rng: &mut SmallRng) -> Self {
        let speed = rng.gen_range(MIN_SNOWFLAKE_SPEED..MAX_SNOWFLAKE_SPEED);
        let color = HexColor::WHITE.scale((speed - MIN_SNOWFLAKE_SPEED) / (MAX_SNOWFLAKE_SPEED - MIN_SNOWFLAKE_SPEED));

        Self { 
            pos:  Vec2::new(
                rng.gen_range(0.0..50.0), 
                rng.gen_range(0.0..24.0)
            ),
            speed,
            color,
        }
    }

    fn new_randon_top(&mut self, rng: &mut SmallRng) {
        self.pos = Vec2::new(
            rng.gen_range(0.0..50.0), 
            0.0
        );
    }
}

struct Snow {
    snowflakes: [SnowFlake; NUM_SNOWFLAKES],
    rng: SmallRng,
}

impl Snow {
    fn new() -> Self {
        let mut rng = SmallRng::seed_from_u64(0);

        Self {
            snowflakes: core::array::from_fn(|_| SnowFlake::new_random(&mut rng)),
            rng,
        }
    }
}

impl<const S: usize, const X: usize, const Y: usize> Render<S, X, Y> for Snow {
    fn step(&mut self) {
        for snowflake in self.snowflakes.iter_mut() {
            snowflake.pos.y += snowflake.speed;
            if snowflake.pos.y > 24.0 {
                snowflake.new_randon_top(&mut self.rng);
            }
        }
    }
    fn render(&self, _t: f32, _dt: f32, buffer: &mut RenderBuffer<S, X, Y>) {
        for snowflake in self.snowflakes.iter() {

            let (y1, y2) = (snowflake.pos.y as u32, (snowflake.pos.y + 1.0) as u32);
            let x = snowflake.pos.x as u32;
            let phase = snowflake.pos.y.fract();


            buffer.safe_set_max_rgb(x, y1, snowflake.color.scale(1.0 - phase));
            if phase>0.0 {
                buffer.safe_set_max_rgb(x, y2, snowflake.color.scale(phase));
            }
        }
    }

}