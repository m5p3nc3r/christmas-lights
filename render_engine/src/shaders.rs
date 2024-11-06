#![allow(non_snake_case)]

use crate::ShaderInput;
use crate::RGB8;
use crate::{Vec2, Vec3};

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

// https://www.shadertoy.com/view/lsX3zr
pub fn hypnotic_rectangles(fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
    // vec2 center = vec2(0.5,0.5);
    const CENTER: Vec2 = Vec2::splat(0.5);
    // float speed = 0.005;
    const SPEED: f32 = 0.005;

    // void mainImage( out vec4 fragColor, in vec2 fragCoord )
    // {
    // float invAr = iResolution.y / iResolution.x;
    let invAr = uniforms.iResolution.x / uniforms.iResolution.y;
    // 	vec2 uv = fragCoord.xy / iResolution.xy;
    let uv = Vec2::new(fragCoord.x, fragCoord.y)
        / Vec2::new(uniforms.iResolution.x, uniforms.iResolution.y);

    // 	float x = (center.x-uv.x);
    let x = CENTER.x - uv.x;
    // 	float y = (center.y-uv.y) * invAr;
    let y = (CENTER.y - uv.y) * invAr;

    // 	float anm = cos(iTime*0.2);
    let anm = (uniforms.iTime * 0.2).cos();

    // 	//float r = -(x*x     + y*y)		* anm;  // Circles
    //let r = -(x * x + y * y) * anm; // Circles
    // 	//float r = -(x*x*x   + y*y*y)		* anm;  // Cubic Shape
    //let r = -(x * x * x + y * y * y) * anm; // Cubic Shape
    // 	float r   = -(x*x*x*x + y*y*y*y)	* anm;  // Rectangles
    let r = -(x * x * x * x + y * y * y * y) * anm;
    // 	float z   = 1.0 + 0.5*sin((r+iTime*speed)/0.0015);
    let z = 1.0 + 0.5 * ((r + uniforms.iTime * SPEED) / 0.0015).sin();

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
