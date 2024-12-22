use bevy::{color::palettes::css::*, prelude::*};


mod config;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)  // Add default plugins (including camera)
        .add_systems(Startup, setup)   // Add startup system to set up the scene
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Spawn the camera for 2D rendering
    commands.spawn(Camera2d);

    let _ = commands
        .spawn((
            Node {
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            BorderRadius::new(Val::Px(10.0),Val::Px(10.0),Val::Px(10.0),Val::Px(10.0)),
            BackgroundColor(YELLOW.into()),
        ))
        .id();
}
