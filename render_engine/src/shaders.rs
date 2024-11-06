#![allow(non_snake_case)]

use crate::{RenderBuffer, RGB8};
use crate::{Vec2, Vec3};

#[allow(non_snake_case)]
pub struct ShaderInput {
    pub iResolution: Vec3,
    pub iTime: f32,
    pub iTimeDelta: f32,
}

pub type MainImageFn = fn(Vec2, &ShaderInput) -> RGB8;


pub struct ShaderEngine {
}

impl ShaderEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render<const S:usize, const X: usize, const Y: usize>(&self, t: f32, dt: f32, b: &mut RenderBuffer<S, X, Y>, shaderFn: &MainImageFn) {
        // Calculate the uniforms
        let uniforms = ShaderInput {
            iResolution: Vec3::new(b.size().x as f32, b.size().y as f32, 0.0),
            iTime: t,
            iTimeDelta: dt,
        };

        for x in 0..b.size().x {
            for y in 0..b.size().y {
                b.set_pixel(x, y, shaderFn(Vec2::new(x as f32, y as f32), &uniforms));
            }
        }
    }
}


pub enum Shader {
    Rainbow,
    HypnoticRectangles,
//    Octograms,
}

impl Shader {
    pub fn to_main_image_fn(&self) -> MainImageFn {
        match self {
            Shader::Rainbow => rainbow,
            Shader::HypnoticRectangles => hypnotic_rectangles,
//            Shader::Octograms => octograms,
        }
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


// pub fn octograms(fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
//     // precision highp float;

//     // float gTime = 0.2;
//     // const float REPEAT = 5.0;
//     //const REPEAT: f32 = 5.0;

//     // // 回転行列
//     // mat2 rot(float a) {
//     let rot = |a: f32| -> Mat3 {
//         // 	float c = cos(a), s = sin(a);
//         // let c = a.cos();
//         // let s = a.sin();
//         // // 	return mat2(c,s,-s,c);
//         // Mat2::from_cols_array(&[c, s, -s, c])
//         // }
//         Mat3::from_rotation_z(a)
//     };

//     // float sdBox( vec3 p, vec3 b )
//     let sdBox = |p: Vec3, b: Vec3| -> f32 {
//         // {
//         // 	vec3 q = abs(p) - b;
//         let q = p.abs() - b;
//         // 	return length(max(q,0.0)) + min(max(q.x,max(q.y,q.z)),0.0);
//         q.max(Vec3::ZERO).length() + q.x.max(q.y.max(q.z)).min(0.0)
//         // }
//     };

//     // float box(vec3 pos, float scale) {
//     let bbox = |pos: Vec3, scale: f32| -> f32 {
//         // 	pos *= scale;
//         let pos = pos * scale;
//         // 	float base = sdBox(pos, vec3(.4,.4,.1)) /1.5;
//         -sdBox(pos, Vec3::new(0.4, 0.4, 0.1)) / 1.5
//         //let result = -base;
//         // 	return result;
//         // }
//     };

//     // float box_set(vec3 pos, float iTime) {
//     let box_set = |pos: Vec3, gTime: f32| -> f32 {
//         // 	vec3 pos_origin = pos;
//         let pos_origin = pos;
//         // 	pos = pos_origin;
//         //let pos = pos_origin;
//         // 	pos .y += sin(gTime * 0.4) * 2.5;
//         let mut pos = pos + Vec3::new(0.0, gTime.sin() * 2.5, 0.0);
//         // 	pos.xy *=   rot(.8);
//         pos = /*pos **/ rot(0.8) * pos;
//         // 	float box1 = box(pos,2. - abs(sin(gTime * 0.4)) * 1.5);
//         let box1 = bbox(pos, 2.0 - (gTime.sin() * 1.5).abs());
//         // 	pos = pos_origin;
//         pos = pos_origin;
//         // 	pos .y -=sin(gTime * 0.4) * 2.5;
//         pos -= Vec3::new(0.0, gTime.sin() * 2.5, 0.0);
//         // 	pos.xy *=   rot(.8);
//         pos = /*pos **/ rot(0.8) * pos;
//         // 	float box2 = box(pos,2. - abs(sin(gTime * 0.4)) * 1.5);
//         let box2 = bbox(pos, 2.0 - (gTime.sin() * 1.5).abs());
//         // 	pos = pos_origin;
//         pos = pos_origin;
//         // 	pos .x +=sin(gTime * 0.4) * 2.5;
//         pos += Vec3::new(gTime.sin() * 2.5, 0.0, 0.0);
//         // 	pos.xy *=   rot(.8);
//         pos = /*pos **/ rot(0.8) * pos;
//         // 	float box3 = box(pos,2. - abs(sin(gTime * 0.4)) * 1.5);
//         let box3 = bbox(pos, 2.0 - (gTime.sin() * 1.5).abs());
//         // 	pos = pos_origin;
//         pos = pos_origin;
//         // 	pos .x -=sin(gTime * 0.4) * 2.5;
//         pos -= Vec3::new(gTime.sin() * 2.5, 0.0, 0.0);
//         // 	pos.xy *=   rot(.8);
//         pos = /*pos **/ rot(0.8) * pos;
//         // 	float box4 = box(pos,2. - abs(sin(gTime * 0.4)) * 1.5);
//         let box4 = bbox(pos, 2.0 - (gTime.sin() * 1.5).abs());
//         // 	pos = pos_origin;
//         pos = pos_origin;
//         // 	pos.xy *=   rot(.8);
//         pos = /*pos **/ rot(0.8) * pos;
//         // 	float box5 = box(pos,.5) * 6.;
//         let box5 = bbox(pos, 0.5) * 6.0;
//         // 	pos = pos_origin;
//         pos = pos_origin;
//         // 	float box6 = box(pos,.5) * 6.;
//         let box6 = bbox(pos, 0.5) * 6.0;
//         // 	float result = max(max(max(max(max(box1,box2),box3),box4),box5),box6);
//         box1.max(box2.max(box3.max(box4.max(box5.max(box6)))))
//         // 	return result;
//         //result
//         // }
//     };

//     // float map(vec3 pos, float iTime) {
//     let map = |pos: Vec3, gTime: f32| -> f32 {
//         // 	vec3 pos_origin = pos;
//         //let pos_origin = pos;
//         // 	float box_set1 = box_set(pos, iTime);
//         /*let box_set1 = */
//         box_set(pos, gTime)
//         // 	return box_set1;
//         //box_set1
//         // }
//     };

//     // void mainImage( out vec4 fragColor, in vec2 fragCoord ) {
//     // 	vec2 p = (fragCoord.xy * 2. - iResolution.xy) / min(iResolution.x, iResolution.y);
//     let p = (fragCoord * 2.0 - uniforms.iResolution.xy())
//         / uniforms.iResolution.x.min(uniforms.iResolution.y);
//     // 	vec3 ro = vec3(0., -0.2 ,iTime * 4.);
//     let ro = Vec3::new(0.0, -0.2, uniforms.iTime * 4.0);
//     // 	vec3 ray = normalize(vec3(p, 1.5));
//     let ray = Vec3::new(p.x, p.y, 1.5).normalize();
//     // 	ray.xy = ray.xy * rot(sin(iTime * .03) * 5.);
//     let ray = {
//         let tmp = rot((uniforms.iTime * 0.03).sin() * 5.0) * ray;
//         Vec3::new(tmp.x, tmp.y, ray.z)
//     };
//     //let ray = Vec3::new(ray.x, ray.y, ray.z).mul_element_wise(rot(uniforms.iTime * 0.03 * 5.0));
//     // 	ray.yz = ray.yz * rot(sin(iTime * .05) * .2);
//     let ray = {
//         let tmp = rot((uniforms.iTime * 0.05).sin() * 0.2) * ray;
//         Vec3::new(ray.x, tmp.y, tmp.z)
//     };
//     //let ray = Vec3::new(ray.x, ray.y, ray.z).mul_element_wise(rot(uniforms.iTime * 0.05 * 0.2));
//     // 	float t = 0.1;
//     let mut t = 0.1;
//     // 	vec3 col = vec3(0.);
//     //let mut col = Vec3::ZERO;
//     // 	float ac = 0.0;
//     let mut ac = 0.0;

//     // 	for (int i = 0; i < 99; i++){
//     for i in 0..30 {
//         // 		vec3 pos = ro + ray * t;
//         let mut pos = ro + ray * t;
//         // 		pos = mod(pos-2., 4.) -2.;
//         pos = (pos - 2.0) % 4.0 - 2.0;
//         //let pos = (pos - 2.0).rem_euclid(4.0) - 2.0;
//         // 		gTime = iTime -float(i) * 0.01;
//         let gTime = uniforms.iTime - i as f32 * 0.01;

//         // 		float d = map(pos, iTime);
//         let mut d = map(pos, gTime);

//         // 		d = max(abs(d), 0.01);
//         d = d.abs().max(0.01);

//         // 		ac += exp(-d*23.);
//         ac += (-d * 23.0).exp();

//         // 		t += d* 0.55;
//         t += d * 0.55;
//         // 	}
//     }

//     // 	col = vec3(ac * 0.02);
//     let mut col = Vec3::splat(ac * 0.02);

//     // 	col +=vec3(0.,0.2 * abs(sin(iTime)),0.5 + sin(iTime) * 0.2);
//     col += Vec3::new(
//         0.0,
//         0.2 * uniforms.iTime.sin().abs(),
//         0.5 + uniforms.iTime.sin() * 0.2,
//     );

//     // 	fragColor = vec4(col ,1.0 - t * (0.02 + 0.02 * sin (iTime)));
//     RGB8 {
//         r: (col.x * 255.0) as u8,
//         g: (col.y * 255.0) as u8,
//         b: (col.z * 255.0) as u8,
//     }
// }
