use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::prelude::*;
use std::time::Duration;

const PLAYER_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const PLAYER_SPEED: f32 = 300.0;
const WALL_THICKNESS: f32 = 60.0;
const ROOM_SIZE: f32 = 500.0;
const PASSAGE_WIDTH: f32 = 10.0;
const COIN_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const COIN_SPEED: f32 = 200.0;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Coin {
    velocity: Vec2,
    stationary_timer: Timer,
}

#[derive(Component)]
struct Room;

// Asset handles
#[derive(Resource, Default)]
struct GameAssets {
    player_sprite: Handle<Image>,
    coin_sprite: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameAssets::default())
        .add_state::<GameState>()
        .add_systems(Startup, load_assets)
        .add_systems(Startup, setup.after(load_assets))
        .add_systems(
            Update,
            (
                player_movement,
                coin_movement,
                check_coin_collision,
                spawn_coins,
                handle_coin_state_change,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    game_assets.player_sprite = asset_server.load("ball.png");
    game_assets.coin_sprite = asset_server.load("coin.png");
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Player
    commands.spawn((
        SpriteBundle {
            texture: game_assets.player_sprite.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            ..default()
        },
        Player,
    ));

    spawn_room(&mut commands, Vec2::ZERO);
}

fn spawn_room(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
        Room,
    ));

    // Walls
    let half_size = ROOM_SIZE / 2.0;
    let half_passage = PASSAGE_WIDTH / 2.0;

    // Top wall (with passage)
    spawn_wall(
        commands,
        position + Vec2::new(-half_size, half_size),
        Vec2::new(half_size - half_passage, WALL_THICKNESS),
    );
    spawn_wall(
        commands,
        position + Vec2::new(half_size, half_size),
        Vec2::new(half_size - half_passage, WALL_THICKNESS),
    );

    // Bottom wall (with passage)
    spawn_wall(
        commands,
        position + Vec2::new(-half_size, -half_size),
        Vec2::new(half_size - half_passage, WALL_THICKNESS),
    );
    spawn_wall(
        commands,
        position + Vec2::new(half_size, -half_size),
        Vec2::new(half_size - half_passage, WALL_THICKNESS),
    );

    // Left wall (with passage)
    spawn_wall(
        commands,
        position + Vec2::new(-half_size, -half_size),
        Vec2::new(WALL_THICKNESS, half_size - half_passage),
    );
    spawn_wall(
        commands,
        position + Vec2::new(-half_size, half_size),
        Vec2::new(WALL_THICKNESS, half_size - half_passage),
    );

    // Right wall (with passage)
    spawn_wall(
        commands,
        position + Vec2::new(half_size, -half_size),
        Vec2::new(WALL_THICKNESS, half_size - half_passage),
    );
    spawn_wall(
        commands,
        position + Vec2::new(half_size, half_size),
        Vec2::new(WALL_THICKNESS, half_size - half_passage),
    );
}

fn spawn_wall(commands: &mut Commands, position: Vec2, size: Vec2) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.8, 0.8),
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        Wall,
    ));
}

fn player_movement(
    mut player_query: Query<(&mut Transform, &Sprite), With<Player>>,
    wall_query: Query<(&Transform, &Sprite), (With<Wall>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, sprite)) = player_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            let new_pos = transform.translation + (direction * PLAYER_SPEED * time.delta_seconds()).extend(0.0);
            
            // Check wall collisions
            let player_size = sprite.custom_size.unwrap_or(Vec2::ONE);
            for (wall_transform, wall_sprite) in wall_query.iter() {
                let wall_size = wall_sprite.custom_size.unwrap_or(Vec2::ONE);
                if let Some(_) = collide(
                    new_pos,
                    player_size,
                    wall_transform.translation,
                    wall_size,
                ) {
                    return;
                }
            }
            
            transform.translation = new_pos;
        }
    }
}

fn spawn_coins(
    mut commands: Commands,
    query: Query<&Transform, With<Room>>,
    game_assets: Res<GameAssets>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.02) {
        // Only spawn if we have a room
        if let Ok(room_transform) = query.get_single() {
            let random_position = Vec2::new(
                rng.gen_range(-ROOM_SIZE/2.0 + 50.0..ROOM_SIZE/2.0 - 50.0),
                rng.gen_range(-ROOM_SIZE/2.0 + 50.0..ROOM_SIZE/2.0 - 50.0),
            );

            commands.spawn((
                SpriteBundle {
                    texture: game_assets.coin_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(COIN_SIZE),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        (room_transform.translation.truncate() + random_position).extend(1.0)
                    ),
                    ..default()
                },
                Coin {
                    velocity: Vec2::ZERO,
                    stationary_timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
                },
            ));
        }
    }
}

fn handle_coin_state_change(
    mut coin_query: Query<(&mut Coin, &mut Transform)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    
    for (mut coin, _transform) in coin_query.iter_mut() {
        if coin.stationary_timer.tick(time.delta()).just_finished() {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            coin.velocity = Vec2::new(angle.cos(), angle.sin()) * COIN_SPEED;
        }
    }
}

fn coin_movement(
    mut param_set: ParamSet<(
        Query<(&Transform, &Sprite), With<Wall>>,
        Query<(&mut Transform, &mut Coin)>,
    )>,
    time: Res<Time>,
) {
    // First, collect all wall data
    let wall_data: Vec<(Vec3, Vec2)> = param_set.p0()
        .iter()
        .map(|(transform, sprite)| (
            transform.translation,
            sprite.custom_size.unwrap_or(Vec2::ONE)
        ))
        .collect();

    // Then update coins
    let mut coin_query = param_set.p1();
    for (mut transform, mut coin) in coin_query.iter_mut() {
        if coin.velocity != Vec2::ZERO {
            let new_pos = transform.translation + (coin.velocity * time.delta_seconds()).extend(0.0);
            
            // Check wall collisions
            let mut should_update = true;
            
            for (wall_pos, wall_size) in wall_data.iter() {
                if let Some(collision) = collide(
                    new_pos,
                    COIN_SIZE,
                    *wall_pos,
                    *wall_size,
                ) {
                    match collision {
                        Collision::Left | Collision::Right => coin.velocity.x *= -1.0,
                        Collision::Top | Collision::Bottom => coin.velocity.y *= -1.0,
                        Collision::Inside => should_update = false,
                    }
                    break;
                }
            }
            
            if should_update {
                transform.translation = new_pos;
            }
        }
    }
}

fn check_coin_collision(
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    coin_query: Query<(&Transform, &Sprite), With<Coin>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((player_transform, player_sprite)) = player_query.get_single() {
        for (coin_transform, coin_sprite) in coin_query.iter() {
            if let Some(_) = collide(
                player_transform.translation,
                player_sprite.custom_size.unwrap_or(Vec2::ONE),
                coin_transform.translation,
                coin_sprite.custom_size.unwrap_or(Vec2::ONE),
            ) {
                next_state.set(GameState::GameOver);
                println!("Game Over!");
                return;
            }
        }
    }
}
