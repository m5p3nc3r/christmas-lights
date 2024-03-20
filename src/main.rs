use bevy::prelude::*;

//
const NUM_DROP: u32 = 50;
const LEDS_PER_DROP: u32 = 24;
const PIXEL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const PIXEL_SPACING: f32 = 1.1;

#[derive(Component)]
struct Pixel {
    index: u32,
}

impl Pixel {
    fn new(index: u32) -> Self {
        Self { index }
    }
}

#[derive(Clone, Copy)]
struct RGB8 {
    r: u8,
    g: u8,
    b: u8,
}
impl Default for RGB8 {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}

#[derive(Resource)]
struct RenderEngine {
    buffer: [RGB8; NUM_DROP as usize * LEDS_PER_DROP as usize],
}

impl RenderEngine {
    fn new() -> Self {
        Self {
            buffer: [RGB8::default(); NUM_DROP as usize * LEDS_PER_DROP as usize],
        }
    }

    fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = RGB8::default();
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: RGB8) {
        let index = x + y * NUM_DROP;
        self.buffer[index as usize] = color;
    }

    fn get_pixel(&self, x: u32, y: u32) -> RGB8 {
        let index = x + y * NUM_DROP;
        self.buffer[index as usize]
    }

    fn render(&mut self, uniforms: &ShaderInput, f: fn(fragCoord: Vec2, &ShaderInput) -> RGB8) {
        for x in 0..NUM_DROP {
            for y in 0..LEDS_PER_DROP {
                self.set_pixel(x, y, f(Vec2::new(x as f32, y as f32), uniforms));
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update_offscreen_render, update_pixels))
        .run();
}

fn setup(mut commands: Commands, windows: Query<&mut Window>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(RenderEngine::new());

    let window = windows.single();
    let width = window.width();
    let height = window.height();
    let pixels_width = NUM_DROP as f32 * PIXEL_SIZE.x * PIXEL_SPACING;
    let pixels_height = LEDS_PER_DROP as f32 * PIXEL_SIZE.y * PIXEL_SPACING;

    let mut x_offset = (width - pixels_width) / 2.0;
    let mut y_offset = (height - pixels_height) / 2.0;
    x_offset = width / 2.0 - x_offset;
    y_offset = height / 2.0 - y_offset;

    let mut index = 0;

    for i in 0..NUM_DROP {
        for j in 0..LEDS_PER_DROP {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            PIXEL_SIZE.x * PIXEL_SPACING * i as f32 - x_offset,
                            -PIXEL_SIZE.y * PIXEL_SPACING * j as f32 + y_offset,
                            0.0,
                        ),
                        scale: PIXEL_SIZE.extend(1.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    ..default()
                },
                Pixel::new(index),
            ));
            index += 1;
        }
    }
}

struct ShaderInput {
    iResolution: Vec3,
    iTime: f32,
    iTimeDelta: f32,
}

fn update_offscreen_render(time: Res<Time>, mut render_engine: ResMut<RenderEngine>) {
    let uniforms = ShaderInput {
        iResolution: Vec3::new(NUM_DROP as f32, LEDS_PER_DROP as f32, 0.0),
        iTime: time.elapsed_seconds(),
        iTimeDelta: time.delta_seconds(),
    };

    //render_engine.render(&uniforms, rainbow);
    render_engine.render(&uniforms, hypnotic_rectangles);
}

fn update_pixels(render_engine: ResMut<RenderEngine>, mut query: Query<(&Pixel, &mut Sprite)>) {
    for (pixel, mut sprite) in query.iter_mut() {
        let x = pixel.index / LEDS_PER_DROP;
        let y = pixel.index % LEDS_PER_DROP;

        let color = render_engine.get_pixel(x, y);
        sprite.color = Color::rgb(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        );
    }
}

fn rainbow(fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
    let offset = fragCoord.y;

    let t = uniforms.iTime + offset / 15.0;

    let r = ((t * 2.0).sin() * 0.5 + 0.5) as f32;
    let g = ((t * 0.7).sin() * 0.5 + 0.5) as f32;
    let b = ((t * 1.3).sin() * 0.5 + 0.5) as f32;

    RGB8 {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
    }
}

// https://www.shadertoy.com/view/lsX3zr
fn hypnotic_rectangles(fragCoord: Vec2, uniforms: &ShaderInput) -> RGB8 {
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
    let x = (0.5 - uv.x);
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
