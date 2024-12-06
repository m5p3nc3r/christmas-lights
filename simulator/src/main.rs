use bevy::prelude::*;
use render_engine::{RenderBuffer, RenderEngine, Renderer, RenderType, Fixed};
use az::Cast;

//
const NUM_DROP: u32 = 50;
const LEDS_PER_DROP: u32 = 24;
const PIXEL_SIZE: Vec2 = Vec2::new(20.0, 20.0);
const PIXEL_SPACING: f32 = 1.1;

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct Pixel {
    index: u32,
}

impl Pixel {
    fn new(index: u32) -> Self {
        Self { index }
    }
}

type Buffer50x24 = RenderBuffer<{50 * 24}, 50, 24>;

#[derive(Resource)]
struct LEDRenderBuffer {
    buffer: Buffer50x24,
}

impl Default for LEDRenderBuffer {
    fn default() -> Self {
        Self {
            buffer: Buffer50x24::new()
        }
    }
}

#[derive(Resource, Default)]
struct LEDRenderEngine {
    engine: RenderEngine,
}

unsafe impl Send for LEDRenderEngine {}
unsafe impl Sync for LEDRenderEngine {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, set_default_shader.after(setup)))
        .add_systems(
            FixedUpdate,
            (update_offscreen_render, update_pixels),
        )
        .add_systems(Update, keyboard_input)

        
        .run();
}

fn setup(mut commands: Commands, windows: Query<&mut Window>) {

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = bevy::render::camera::ScalingMode::FixedHorizontal(1280.0);

    // Camera
    commands.spawn((camera, GameCamera));

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
    r.engine.set_renderer(Renderer::Basic(RenderType::Sparkle));
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut r: ResMut<LEDRenderEngine>) {
    if keys.just_pressed(KeyCode::Digit1) {
        r.engine.set_transition_to_renderer(Renderer::Basic(RenderType::Sparkle), Fixed::from_num(1.0));
    } else if keys.just_pressed(KeyCode::Digit2) {
        r.engine.set_transition_to_renderer(Renderer::Basic(RenderType::Snow), Fixed::from_num(1.0));
    // } else if keys.just_pressed(KeyCode::Digit3) {
    //     r.engine.set_transition_to_renderer(Renderer::Shader(Shader::Rainbow), Fixed::from_num(1.0));
    // } else if keys.just_pressed(KeyCode::Digit4) {
    //     r.engine
    //         .set_transition_to_renderer(Renderer::Shader(Shader::HypnoticRectangles), Fixed::from_num(1.0));
    }
}

fn update_offscreen_render(
    time: Res<Time>,
    mut r: ResMut<LEDRenderEngine>,
    mut b: ResMut<LEDRenderBuffer>,
) {
    // TODO: Fix elapsed_seconds so that it wraps 
    r.engine.render(Fixed::ZERO/*Fixed::from_num(time.elapsed_seconds())*/, Fixed::from_num(time.delta_seconds()), &mut b.buffer);
}

fn update_pixels(r: Res<LEDRenderEngine>, b: ResMut<LEDRenderBuffer>, mut pixels: Query<(&Pixel, &mut Sprite)>) {
    for (pixel, mut sprite) in pixels.iter_mut() {
        let x = pixel.index / LEDS_PER_DROP;
        let y = pixel.index % LEDS_PER_DROP;

        let color = r.engine.get_render_buffer().get_pixel(x, y);
        sprite.color = Color::srgb(
            color.r.cast(),
            color.g.cast(),
            color.b.cast(),
        );
    }
}
