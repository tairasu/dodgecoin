use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub player_sprite: Handle<Image>,
    pub coin_sprite: Handle<Image>,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self {
            player_sprite: Handle::default(),
            coin_sprite: Handle::default(),
        }
    }
}

#[derive(Resource, Default)]
pub struct GameTimer {
    pub elapsed_secs: f32,
} 