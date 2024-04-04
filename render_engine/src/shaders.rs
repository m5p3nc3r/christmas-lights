#![allow(non_snake_case)]

use crate::ShaderInput;
use crate::RGB8;
use crate::{Vec2, Vec3};

pub trait ShaderPass {
    fn step(&mut self) {}
    fn mainImage(&self, fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8;
}

pub struct Rainbow {}

impl ShaderPass for Rainbow {
    fn mainImage(&self, fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
        rainbow(fragCoord, uniforms)
    }
}

pub fn rainbow(fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
    let offset = fragCoord.y;

    let t = uniforms.iTime + offset / 15.0;

    let r = (t * 2.0).sin() * 0.5 + 0.5;
    let g = (t * 0.7).sin() * 0.5 + 0.5;
    let b = (t * 1.3).sin() * 0.5 + 0.5;

    RGB8 {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
    }
}

pub struct HypnoticRectanges {}

impl ShaderPass for HypnoticRectanges {
    fn mainImage(&self, fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
        hypnotic_rectangles(fragCoord, uniforms)
    }
}

// https://www.shadertoy.com/view/lsX3zr
pub fn hypnotic_rectangles(fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
    // vec2 center = vec2(0.5,0.5);
    // float speed = 0.005;

    // void mainImage( out vec4 fragColor, in vec2 fragCoord )
    // {
    // float invAr = iResolution.y / iResolution.x;
    let invAr = uniforms.iResolution.x / uniforms.iResolution.y;
    // 	vec2 uv = fragCoord.xy / iResolution.xy;
    let uv = Vec2::new(fragCoord.x, fragCoord.y)
        / Vec2::new(uniforms.iResolution.x, uniforms.iResolution.y);

    // 	float x = (center.x-uv.x);
    let x = 0.5 - uv.x;
    // 	float y = (center.y-uv.y) * invAr;
    let y = (0.5 - uv.y) * invAr;

    // 	float anm = cos(iTime*0.2);
    let anm = (uniforms.iTime * 0.2).cos();

    // 	//float r = -(x*x     + y*y)		* anm;  // Circles
    // 	//float r = -(x*x*x   + y*y*y)		* anm;  // Cubic Shape
    // 	float r   = -(x*x*x*x + y*y*y*y)	* anm;  // Rectangles
    let r = -(x * x * x * x + y * y * y * y) * anm;
    // 	float z   = 1.0 + 0.5*sin((r+iTime*speed)/0.0015);
    let z = 1.0 + 0.5 * ((r + uniforms.iTime * 0.005) / 0.0015).sin();

    // 	//Color
    // 	vec3 col = vec4(uv,0.5+0.5*sin(iTime),1.0).xyz;
    let col = Vec3::new(uv.x, uv.y, 0.5 + 0.5 * uniforms.iTime.sin());
    // 	vec3 texcol = vec3(z,z,z);
    let texcol = Vec3::splat(z);

    // 	fragColor = vec4(col*texcol,1.0);
    RGB8 {
        r: (col.x * texcol.x * 255.0) as u8,
        g: (col.y * texcol.y * 255.0) as u8,
        b: (col.z * texcol.z * 255.0) as u8,
    }
}
// }

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Snow {
    snowflakes: [(f32, f32); 100], // Static array of 100 snowflakes
}

impl Default for Snow {
    fn default() -> Self {
        Self::new()
    }
}
impl Snow {
    pub fn new() -> Self {
        let mut rng = SmallRng::seed_from_u64(0);
        Self {
            snowflakes: core::array::from_fn(|_| {
                (rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0))
            }),
        }
    }
}

impl ShaderPass for Snow {
    fn step(&mut self) {
        // Move each snowflake down by a small amount
        for i in 0..100 {
            self.snowflakes[i].1 += 0.01;
            if self.snowflakes[i].1 > 1.0 {
                self.snowflakes[i].1 -= 1.0;
            }
        }
    }

    fn mainImage(&self, fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
        // If the current pixel is close to a snowflake, draw a snowflake
        if self.snowflakes.iter().any(|(x, y)| {
            let dx = fragCoord.x - x * uniforms.iResolution.x;
            let dy = fragCoord.y - y * uniforms.iResolution.y;
            dx * dx + dy * dy < 1.0
        }) {
            RGB8 {
                r: 255,
                g: 255,
                b: 255,
            }
        } else {
            RGB8 { r: 0, g: 0, b: 0 }
        }

        // // Calculate the "height" of the snowflake based on time and the y-coordinate
        // let height = ((fragCoord.y - uniforms.iTime * 10.0) % uniforms.iResolution.y)
        //     / uniforms.iResolution.y;

        // // Generate multiple noise values for each y-coordinate
        // let mut max_noise: f32 = 0.0;
        // for i in 0..10 {
        //     // Generate random noise based on the x-coordinate, height, and i
        //     let noise =
        //         ((fragCoord.x + (height + i as f32 * 0.1) * 43_758.545).sin() * 43_758.545).fract();
        //     max_noise = max_noise.max(noise);
        // }

        // // If the maximum noise is above a threshold, draw a snowflake
        // if max_noise > 0.98 {
        //     RGB8 {
        //         r: 255,
        //         g: 255,
        //         b: 255,
        //     }
        // } else {
        //     RGB8 { r: 0, g: 0, b: 0 }
        // }
    }

    // fn mainImage(&self, fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
    //     if fragCoord.x == 0.0 {
    //         //            let v = (uniforms.iTime.rem(24.0) * 10.0) as u8;
    //         let v = uniforms.iTime.rem(uniforms.iResolution.y).round();
    //         if fragCoord.y == v {
    //             return RGB8 {
    //                 r: 255,
    //                 g: 255,
    //                 b: 255,
    //             };
    //         }
    //     }
    //     return RGB8 {
    //         r: 128,
    //         g: 128,
    //         b: 128,
    //     };
    // }
}
