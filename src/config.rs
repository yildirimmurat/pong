use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const BORDER_THICKNESS: f32 = 2.0;
const PADDLE_THICKNESS: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 100.0;
const OFFSET: f32 = 10.0; // Some space all around the table

#[derive(Debug)]
pub struct EntityConfig {
    pub shape_size: (f32, f32),
    pub color: Color,
    pub transform: Transform,
    pub z_index: i32,
}

pub fn get_entity_configs() -> Vec<EntityConfig> {
    let color_black = Color::srgb(0.0, 0.0, 0.0);
    let color_white = Color::srgb(1.0, 1.0, 1.0);
    let color_green = Color::srgb(0.0, 1.0, 0.0);

    vec![
        // Background
        EntityConfig {
            shape_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            color: color_black,
            transform: Transform::from_xyz(0.0,0.0,0.0),
            z_index: -1,
        },
        // Left Border
        EntityConfig {
            shape_size: (BORDER_THICKNESS, WINDOW_HEIGHT - 2.0 * OFFSET),
            color: color_white,
            transform: Transform::from_xyz(
                OFFSET + BORDER_THICKNESS / 2.0 - WINDOW_WIDTH / 2.0,
                0.0,
                0.0,
            ),
            z_index: 1,
        },
        // Right Border
        EntityConfig {
            shape_size: (BORDER_THICKNESS, WINDOW_HEIGHT - 2.0 * OFFSET),
            color: color_white,
            transform: Transform::from_xyz(
                WINDOW_WIDTH / 2.0 - BORDER_THICKNESS / 2.0 - OFFSET,
                0.0,
                0.0,
            ),
            z_index: 1,
        },
        // Middle Net
        EntityConfig {
            shape_size: (BORDER_THICKNESS, WINDOW_HEIGHT - 2.0 * OFFSET),
            color: color_white,
            transform: Transform::from_xyz(
                0.0,
                0.0,
                0.0,
            ),
            z_index: 1,
        },
        // Top Border
        EntityConfig {
            shape_size: (WINDOW_WIDTH - 2.0 * OFFSET, BORDER_THICKNESS),
            color: color_white,
            transform: Transform::from_xyz(
                0.0,
                OFFSET + BORDER_THICKNESS / 2.0 - WINDOW_HEIGHT / 2.0,
                0.0,
            ),
            z_index: 1,
        },
        // Bottom Border
        EntityConfig {
            shape_size: (WINDOW_WIDTH - 2.0 * OFFSET, BORDER_THICKNESS),
            color: color_white,
            transform: Transform::from_xyz(
                0.0,
                WINDOW_HEIGHT / 2.0 - BORDER_THICKNESS / 2.0 - OFFSET,
                0.0,
            ),
            z_index: 1,
        },
        // Left Paddle
        EntityConfig {
            shape_size: (PADDLE_THICKNESS, PADDLE_HEIGHT),
            color: color_green,
            transform: Transform::from_xyz(
                OFFSET + BORDER_THICKNESS + PADDLE_THICKNESS / 2.0 - WINDOW_WIDTH / 2.0,
                0.0,
                0.0,
            ),
            z_index: 1,
        },
        // Right Paddle
        EntityConfig {
            shape_size: (PADDLE_THICKNESS, PADDLE_HEIGHT),
            color: color_green,
            transform: Transform::from_xyz(
                WINDOW_WIDTH / 2.0 - BORDER_THICKNESS - PADDLE_THICKNESS / 2.0 - OFFSET,
                0.0,
                0.0,
            ),
            z_index: 1,
        },
    ]
}
