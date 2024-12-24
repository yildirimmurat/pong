use bevy::prelude::*;
use bevy::time::Time; // Import Time from the new location

// Constants
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 500.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

const WALL_THICKNESS: f32 = 2.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const WALL_COLOR: Color = Color::srgb(0.8, 0.8, 0.8);

const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 120.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 10.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_PADDING: f32 = 10.0;

const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
#[require(Sprite, Transform)]
struct Wall;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
    Middle,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
            WallLocation::Middle => Vec2::new(0., 0.),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right | WallLocation::Middle => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl Wall {
    fn new(location: WallLocation) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(WALL_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(0.0),
                scale: location.size().extend(1.0),
                ..default()
            },
        )
    }
}

enum PaddleLocation {
    Left,
    Right,
}

impl PaddleLocation {
    fn position(&self) -> Vec2 {
        match self {
            PaddleLocation::Left => Vec2::new(LEFT_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR + PADDLE_SIZE.x / 2.0, 0.),
            PaddleLocation::Right => Vec2::new(RIGHT_WALL - GAP_BETWEEN_PADDLE_AND_FLOOR - PADDLE_SIZE.x / 2.0, 0.),
        }
    }
}

impl Paddle {
    fn new(location: PaddleLocation) -> (Paddle, Sprite, Transform) {
        (
            Paddle,
            Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
            Transform {
                translation: location.position().extend(1.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            }
        )
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)  // Add Bevy's default plugins (including Time)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                apply_velocity,
                move_paddle,
                check_for_collisions,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Paddles
    commands.spawn(Paddle::new(PaddleLocation::Left));
    commands.spawn(Paddle::new(PaddleLocation::Right));

    // Ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform::from_translation(BALL_STARTING_POSITION)
            .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // Walls
    commands.spawn(Wall::new(WallLocation::Left));
    commands.spawn(Wall::new(WallLocation::Right));
    commands.spawn(Wall::new(WallLocation::Bottom));
    commands.spawn(Wall::new(WallLocation::Top));
    commands.spawn(Wall::new(WallLocation::Middle));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,  // Input resource for detecting key presses
    mut paddle_query: Query<&mut Transform, With<Paddle>>,  // Query both paddles
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction -= 1.0;
    }

    // We know the left paddle is at index 0
    if let Some(mut paddle_transform) = paddle_query.iter_mut().next() {
        // Calculate the new position based on user input and speed
        let new_position = paddle_transform.translation.y + direction * PADDLE_SPEED * time.delta_secs();

        // Restrict the paddle within the vertical boundaries of the screen
        let upper_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;
        let lower_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;

        // Set the new position within the bounds
        paddle_transform.translation.y = new_position.clamp(lower_bound, upper_bound);
    }
}

fn check_for_collisions(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,  // Query for Ball
    paddle_query: Query<(&Transform, &Paddle), Without<Ball>>, // Query for Paddle (excluding Ball)
) {
    for (ball_transform, mut velocity) in &mut ball_query {
        let ball_pos = ball_transform.translation.truncate();
        let ball_radius = BALL_DIAMETER / 2.0;

        // Wall Collision (Left, Right, Top, Bottom)
        if ball_pos.x - ball_radius <= LEFT_WALL || ball_pos.x + ball_radius >= RIGHT_WALL {
            velocity.x = -velocity.x; // Reflect horizontally
        }

        if ball_pos.y - ball_radius <= BOTTOM_WALL || ball_pos.y + ball_radius >= TOP_WALL {
            velocity.y = -velocity.y; // Reflect vertically
        }

        // Paddle Collision
        for (paddle_transform, _) in &paddle_query {
            let paddle_pos = paddle_transform.translation.truncate();
            let paddle_half_width = PADDLE_SIZE.x / 2.0;
            let paddle_half_height = PADDLE_SIZE.y / 2.0;

            // Check if the ball is colliding with the paddle
            if ball_pos.x - ball_radius < paddle_pos.x + paddle_half_width
                && ball_pos.x + ball_radius > paddle_pos.x - paddle_half_width
                && ball_pos.y - ball_radius < paddle_pos.y + paddle_half_height
                && ball_pos.y + ball_radius > paddle_pos.y - paddle_half_height
            {
                // Ball collided with the paddle - reverse the x direction
                velocity.x = -velocity.x;
            }
        }
    }
}

