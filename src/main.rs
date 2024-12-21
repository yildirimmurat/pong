use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        #[cfg(not(target_arch = "wasm32"))]
        Wireframe2dPlugin,
    ))
        .add_systems(Startup, setup);
    app.run();
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const NET_WIDTH: f32 = 10.0; // The width of the vertical line (the net)
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the camera
    commands.spawn(Camera2d);

    // Create the Pong table (background)
    let background = meshes.add(Rectangle::new(WINDOW_WIDTH, WINDOW_HEIGHT));
    commands.spawn((
        Mesh2d(background),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::default(),
    ));

    // Create a vertical line in the middle (the net)
    let net = meshes.add(Rectangle::new(NET_WIDTH, WINDOW_HEIGHT));
    commands.spawn((
        Mesh2d(net),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0), // Center the net in the middle of the screen
            ..default()
        },
    ));

    // Create left paddle
    let left_paddle = meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    commands.spawn((
        Mesh2d(left_paddle),
        MeshMaterial2d(materials.add(Color::hsla(120.0, 1.0, 0.5, 1.0))),
        Transform {
            translation: Vec3::new(-(WINDOW_WIDTH / 2.0) + PADDLE_WIDTH / 2.0, 0.0, 0.0),
            ..default()
        },
    ));

    // Create right paddle
    let right_paddle = meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT));
    commands.spawn((
        Mesh2d(right_paddle),
        MeshMaterial2d(materials.add(Color::hsla(120.0, 1.0, 0.5, 1.0))),
        Transform {
            translation: Vec3::new(WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0, 0.0, 0.0),
            ..default()
        },
    ));
}
