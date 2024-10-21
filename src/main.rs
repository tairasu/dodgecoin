use bevy::prelude::*;
use rand::prelude::*;

struct Player;
struct Coin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player_movement.system())
        .add_system(coin_collision.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut textures: ResMut<Assets<Texture>>, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Spawn player
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load("assets/player.png").into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    })
    .insert(Player);

    // Spawn coins
    for _ in 0..10 {
        let x = rand::thread_rng().gen_range(-300.0..300.0);
        let y = rand::thread_rng().gen_range(-300.0..300.0);
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(asset_server.load("assets/coin.png").into()),
            transform: Transform::from_xyz(x, y, 0.0),
            ..Default::default()
        })
        .insert(Coin);
    }
}

fn player_movement(mut query: Query<(&Player, &mut Transform)>, keyboard_input: Res<Input<KeyCode>>, time: Res<Time>) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        transform.translation += direction * time.delta_seconds() * 500.0;
    }
}

fn coin_collision(mut commands: Commands, mut player_query: Query<(&Player, &Transform)>, mut coin_query: Query<(Entity, &Coin, &Transform)>) {
    for (_, player_transform) in player_query.iter_mut() {
        for (coin_entity, _, coin_transform) in coin_query.iter_mut() {
            let distance = player_transform.translation.distance(coin_transform.translation);
            if distance < 30.0 {
                commands.entity(coin_entity).despawn();
            }
        }
    }
}
