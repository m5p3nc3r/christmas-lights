use bevy::prelude::*;

//
const NUM_DROP: u32 = 50;
const LEDS_PER_DROP: u32 = 24;

#[derive(Component)]
struct Pixel {
    index: u32,
}

impl Pixel {
    fn new(index: u32) -> Self {
        Self { index }
    }
}

const PIXEL_SIZE: Vec2 = Vec2::new(20.0, 20.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update_offscreen_render, update_pixels))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    let mut index = 0;

    for i in 0..NUM_DROP {
        for j in 0..LEDS_PER_DROP {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            PIXEL_SIZE.x * 1.1 * i as f32,
                            -PIXEL_SIZE.y * 1.1 * j as f32,
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

fn update_offscreen_render() {}

fn update_pixels(time: Res<Time>, mut query: Query<(&Pixel, &mut Sprite)>) {
    let time_seconds = time.elapsed_seconds();

    for (pixel, mut sprite) in query.iter_mut() {
        let offset = (pixel.index % LEDS_PER_DROP) as f32;
        //        let offset = (pixel.index / LEDS_PER_DROP) as f32;
        let t = time_seconds + offset / 15.0;

        let r = ((t * 2.0).sin() * 0.5 + 0.5) as f32;
        let g = ((t * 0.7).sin() * 0.5 + 0.5) as f32;
        let b = ((t * 1.3).sin() * 0.5 + 0.5) as f32;

        sprite.color = Color::rgb(r, g, b);
    }
}
