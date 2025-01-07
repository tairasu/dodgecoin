use bevy::prelude::*;

pub const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const PLAYER_SPEED: f32 = 300.0;
pub const WALL_THICKNESS: f32 = 20.0;
pub const ROOM_SIZE: f32 = 600.0;
pub const PASSAGE_WIDTH: f32 = 100.0;
pub const COIN_SIZE: Vec2 = Vec2::new(24.0, 24.0);
pub const COIN_SPEED: f32 = 200.0;
pub const COIN_SPAWN_CHANCE: f32 = 0.02;
pub const COIN_STATIONARY_TIME: f32 = 3.0;
pub const HOLE_CHANCE: f32 = 0.99;
pub const ROOM_GENERATION_DISTANCE: i32 = 2;
// Add a small buffer to prevent coins from popping in/out at screen edges
pub const VISIBILITY_BUFFER: f32 = 100.0; 