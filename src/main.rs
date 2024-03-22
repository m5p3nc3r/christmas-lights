use bevy::prelude::*;
use render_engine::{
    shaders::{HypnoticRectanges, Rainbow},
    RenderBuffer, RenderEngine, ShaderInput, RGB8,
};

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

struct Buffer50x24 {
    buffer: [RGB8; NUM_DROP as usize * LEDS_PER_DROP as usize],
}

impl RenderBuffer for Buffer50x24 {
    fn size(&self) -> render_engine::Vec2 {
        render_engine::Vec2::new(NUM_DROP as f32, LEDS_PER_DROP as f32)
    }

    fn buffer(&self) -> &[RGB8] {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut [RGB8] {
        &mut self.buffer
    }
}

#[derive(Resource)]
struct LEDRenderBuffer {
    buffer: Buffer50x24,
}

impl Default for LEDRenderBuffer {
    fn default() -> Self {
        Self {
            buffer: Buffer50x24 {
                buffer: [RGB8::default(); NUM_DROP as usize * LEDS_PER_DROP as usize],
            },
        }
    }
}

#[derive(Resource, Default)]
struct LEDRenderEngine {
    engine: RenderEngine<'static>,
}

unsafe impl Send for LEDRenderEngine {}
unsafe impl Sync for LEDRenderEngine {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, set_default_shader.after(setup)))
        .add_systems(
            FixedUpdate,
            (keyboard_input, update_offscreen_render, update_pixels),
        )
        .run();
}

fn setup(mut commands: Commands, windows: Query<&mut Window>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    commands.init_resource::<LEDRenderBuffer>();
    commands.init_resource::<LEDRenderEngine>();

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

fn set_default_shader(mut r: ResMut<LEDRenderEngine>) {
    //    r.engine.set_shader(&Hypnotic_Rectanges {});
    r.engine.set_shader(&Rainbow {});
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut r: ResMut<LEDRenderEngine>) {
    if keys.just_pressed(KeyCode::Digit1) {
        r.engine.set_transition_to_shader(&Rainbow {}, 1.0);
    } else if keys.just_pressed(KeyCode::Digit2) {
        r.engine
            .set_transition_to_shader(&HypnoticRectanges {}, 1.0);
    }
}

fn update_offscreen_render(
    time: Res<Time>,
    mut r: ResMut<LEDRenderEngine>,
    mut b: ResMut<LEDRenderBuffer>,
) {
    let uniforms = ShaderInput {
        iResolution: b.buffer.size().extend(0.0),
        iTime: time.elapsed_seconds(),
        iTimeDelta: time.delta_seconds(),
    };

    //    r.engine.set_shader(&shaders::hypnotic_rectangles);
    r.engine.render(&uniforms, &mut b.buffer);
}

fn update_pixels(b: ResMut<LEDRenderBuffer>, mut query: Query<(&Pixel, &mut Sprite)>) {
    for (pixel, mut sprite) in query.iter_mut() {
        let x = pixel.index / LEDS_PER_DROP;
        let y = pixel.index % LEDS_PER_DROP;

        let color = b.buffer.get_pixel(x, y);
        sprite.color = Color::rgb(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
        );
    }
}
