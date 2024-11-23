use crate::UVec2;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use az::Cast;

struct FixedVec2 {
    x: Fixed,
    y: Fixed,
}

// A macro that generates a generic typed fixed-point number from a smallrng
macro_rules! fixed_rng_gen {
    ($rng:expr, $type:ty) => {
        {
            <$type>::from_bits($rng.gen())
        }
    };
}

macro_rules! fixed_rng_gen_range {
    ($rng:expr, $type:ty, $min:expr, $max:expr) => {
        {
            let min = <$type>::from_num($min).to_bits();
            let max = <$type>::from_num($max).to_bits();
            let val = $rng.gen_range(min..max);
            <$type>::from_bits(val)
        }
    };
}



use crate::fixedcolor::FixedColor;
use crate::{Fixed, RenderBuffer};

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

    pub fn render(&self, renderer: RenderType, t: Fixed, dt: Fixed, buffer: &mut RenderBuffer<S, X, Y>) {
        match renderer {
            RenderType::Sparkle => self.sparkle.render(t, dt, buffer),
            RenderType::Snow => self.snow.render(t, dt, buffer),
        }
    }
}



pub trait Render<const S: usize, const X: usize, const Y: usize> {
    fn step(&mut self);
    fn render(&self, t: Fixed, dt: Fixed, buffer: &mut RenderBuffer<S, X, Y>);
}


type SparklePhase = fixed::FixedU8<fixed::types::extra::U8>;

#[derive(Clone, Copy)]
struct SparklePoint {
    pos: UVec2,
    color: FixedColor,
    phase: SparklePhase,
}

impl SparklePoint {
    fn random_pos(rng: &mut SmallRng) -> Self {

        let phase = fixed_rng_gen!(rng, SparklePhase);

        Self {
            pos: UVec2::new(rng.gen_range(0..50), rng.gen_range(0..24)),
            color: FixedColor::WHITE,
            phase,
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
        let phase_inc = SparklePhase::from_num(0.005);
        for point in self.points.iter_mut() {
            if let Some(phase) = point.phase.checked_add(phase_inc) {
                point.phase = phase;
            } else {
                *point = SparklePoint::random_pos(&mut self.rng);
                point.phase = SparklePhase::default();
            }
        }
    }

    fn render(&self, _t: Fixed, _dt: Fixed, buffer: &mut RenderBuffer<S, X, Y>) {
        for point in self.points.iter() {
            let colour = point.color.scale(point.phase.cast());
            buffer.safe_set_pixel(point.pos.x, point.pos.y, colour);
        }
    }
}
// -----

const NUM_SNOWFLAKES: usize = 200;
const MAX_SNOWFLAKE_SPEED: f32 = 0.5;
const MIN_SNOWFLAKE_SPEED: f32 = 0.1;
struct SnowFlake {
    pos: FixedVec2,
    speed: Fixed,
    color: FixedColor,
}

impl SnowFlake {
    fn new_random(rng: &mut SmallRng) -> Self {
        let min = Fixed::from_num(MIN_SNOWFLAKE_SPEED);
        let max = Fixed::from_num(MAX_SNOWFLAKE_SPEED);

        let speed = fixed_rng_gen_range!(rng, Fixed, MIN_SNOWFLAKE_SPEED, MAX_SNOWFLAKE_SPEED);
        let scale = (speed - min) / (max - min);
        let color = FixedColor::WHITE.scale(scale);


        Self { 
            pos:  FixedVec2 {
                x: fixed_rng_gen_range!(rng, Fixed, 0.0, 50.0),
                y: fixed_rng_gen_range!(rng, Fixed, 0.0, 24.0),
            },
            speed,
            color,
        }
    }

    fn new_randon_top(&mut self, rng: &mut SmallRng) {
        self.pos = FixedVec2 {
            x: fixed_rng_gen_range!(rng, Fixed, 0.0, 50.0),
            y: Fixed::ZERO,
        };
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
    fn render(&self, _t: Fixed, _dt: Fixed, buffer: &mut RenderBuffer<S, X, Y>) {
        for snowflake in self.snowflakes.iter() {

            let one = Fixed::ONE;
            let phase = snowflake.pos.y.frac();

            let x: u32 = snowflake.pos.x.cast();
            let y: u32 = snowflake.pos.y.cast();

            buffer.safe_set_max_rgb(x, y, snowflake.color.scale(one - phase));
            if phase >0.0 {
                buffer.safe_set_max_rgb(x, y+1, snowflake.color.scale(phase));
            }
        }
    }

}