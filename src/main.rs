use bevy::prelude::*;
mod components;
mod constants;
mod systems;
mod resources;
mod embedded_assets;

use components::*;
//use constants::*;
use systems::*;
use resources::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dodgecoin".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameAssets>()
        .init_resource::<GameTimer>()
        .add_state::<GameState>()
        .add_systems(Startup, load_assets)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(OnEnter(GameState::Playing), (setup, setup_ui))
        .add_systems(OnEnter(GameState::GameOver), show_game_over)
        .add_systems(OnExit(GameState::GameOver), cleanup_system)
        .add_systems(
            Update,
            (
                handle_buttons,
                (
                    player_movement,
                    spawn_coins,
                    handle_coin_state_change,
                    coin_movement,
                    check_coin_collision,
                    camera_follow,
                    check_room_generation,
                    despawn_invisible_coins,
                    update_timer,
                ).run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}
