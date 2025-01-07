use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameAssets {
    pub player_sprite: Handle<Image>,
    pub coin_sprite: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct GameTimer {
    pub elapsed_secs: f32,
} 