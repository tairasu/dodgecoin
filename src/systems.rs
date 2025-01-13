use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    app::AppExit,
    render::texture::{CompressedImageFormats, ImageType, ImageSampler},
};
use rand::prelude::*;
use std::time::Duration;

use crate::{
    components::{Player, Wall, MainCamera, Coin, Room, Direction, GameState, GameOverText, MenuUI, ButtonAction, Cleanup, MenuCleanup},
    constants::*,
    resources::*,
    embedded_assets::{BALL_BYTES, COIN_BYTES},
};

pub fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    mut image_assets: ResMut<Assets<Image>>,
) {
    // Load player sprite from embedded bytes
    let player_image = Image::from_buffer(
        BALL_BYTES,
        ImageType::Extension("png"),
        CompressedImageFormats::default(),
        true,
        ImageSampler::default(),
    ).unwrap();
    game_assets.player_sprite = image_assets.add(player_image);

    // Load coin sprite from embedded bytes
    let coin_image = Image::from_buffer(
        COIN_BYTES,
        ImageType::Extension("png"),
        CompressedImageFormats::default(),
        true,
        ImageSampler::default(),
    ).unwrap();
    game_assets.coin_sprite = image_assets.add(coin_image);
}

pub fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Camera
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        Cleanup, // Add this to allow cleanup on state exit
    ));

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
        Cleanup,
    ));

    // Initial room
    spawn_room(&mut commands, Vec2::ZERO, Direction::all());
}

pub fn spawn_room(commands: &mut Commands, position: Vec2, exits: Vec<Direction>) {
    let mut rng = rand::thread_rng();
    
    // Filter exits based on HOLE_CHANCE
    let actual_exits: Vec<Direction> = exits.into_iter()
        .filter(|_| rng.gen_bool(HOLE_CHANCE as f64))
        .collect();

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
        Room {
            position,
            exits: actual_exits.clone(),
        },
        Cleanup,
    ));

    let half_size = ROOM_SIZE / 2.0;
    let half_passage = PASSAGE_WIDTH / 2.0;

    // Spawn walls based on exits
    let mut wall_segments = Vec::new();

    // Top wall
    if !actual_exits.contains(&Direction::North) {
        wall_segments.push((
            position + Vec2::new(0.0, half_size),
            Vec2::new(ROOM_SIZE, WALL_THICKNESS),
        ));
    } else {
        // Left part
        wall_segments.push((
            position + Vec2::new(-half_size + (ROOM_SIZE - PASSAGE_WIDTH) / 4.0, half_size),
            Vec2::new((ROOM_SIZE - PASSAGE_WIDTH) / 2.0, WALL_THICKNESS),
        ));
        // Right part
        wall_segments.push((
            position + Vec2::new(half_size - (ROOM_SIZE - PASSAGE_WIDTH) / 4.0, half_size),
            Vec2::new((ROOM_SIZE - PASSAGE_WIDTH) / 2.0, WALL_THICKNESS),
        ));
    }

    // Bottom wall
    if !actual_exits.contains(&Direction::South) {
        wall_segments.push((
            position + Vec2::new(0.0, -half_size),
            Vec2::new(ROOM_SIZE, WALL_THICKNESS),
        ));
    } else {
        // Left part
        wall_segments.push((
            position + Vec2::new(-half_size + (ROOM_SIZE - PASSAGE_WIDTH) / 4.0, -half_size),
            Vec2::new((ROOM_SIZE - PASSAGE_WIDTH) / 2.0, WALL_THICKNESS),
        ));
        // Right part
        wall_segments.push((
            position + Vec2::new(half_size - (ROOM_SIZE - PASSAGE_WIDTH) / 4.0, -half_size),
            Vec2::new((ROOM_SIZE - PASSAGE_WIDTH) / 2.0, WALL_THICKNESS),
        ));
    }

    // Left wall
    if !actual_exits.contains(&Direction::West) {
        wall_segments.push((
            position + Vec2::new(-half_size, 0.0),
            Vec2::new(WALL_THICKNESS, ROOM_SIZE),
        ));
    } else {
        // Top part
        wall_segments.push((
            position + Vec2::new(-half_size, half_size - (ROOM_SIZE - PASSAGE_WIDTH) / 4.0),
            Vec2::new(WALL_THICKNESS, (ROOM_SIZE - PASSAGE_WIDTH) / 2.0),
        ));
        // Bottom part
        wall_segments.push((
            position + Vec2::new(-half_size, -half_size + (ROOM_SIZE - PASSAGE_WIDTH) / 4.0),
            Vec2::new(WALL_THICKNESS, (ROOM_SIZE - PASSAGE_WIDTH) / 2.0),
        ));
    }

    // Right wall
    if !actual_exits.contains(&Direction::East) {
        wall_segments.push((
            position + Vec2::new(half_size, 0.0),
            Vec2::new(WALL_THICKNESS, ROOM_SIZE),
        ));
    } else {
        // Top part
        wall_segments.push((
            position + Vec2::new(half_size, half_size - (ROOM_SIZE - PASSAGE_WIDTH) / 4.0),
            Vec2::new(WALL_THICKNESS, (ROOM_SIZE - PASSAGE_WIDTH) / 2.0),
        ));
        // Bottom part
        wall_segments.push((
            position + Vec2::new(half_size, -half_size + (ROOM_SIZE - PASSAGE_WIDTH) / 4.0),
            Vec2::new(WALL_THICKNESS, (ROOM_SIZE - PASSAGE_WIDTH) / 2.0),
        ));
    }

    // Spawn all wall segments
    for (pos, size) in wall_segments {
        spawn_wall(commands, pos, size);
    }
}

pub fn spawn_wall(commands: &mut Commands, position: Vec2, size: Vec2) {
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
        Cleanup,
    ));
}

pub fn player_movement(
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
            
            // Try to move in both directions independently
            let mut new_pos = transform.translation;
            let delta = PLAYER_SPEED * time.delta_seconds();
            
            // Try X movement
            if direction.x != 0.0 {
                let x_pos = new_pos + Vec3::new(direction.x * delta, 0.0, 0.0);
                let mut x_blocked = false;
                for (wall_transform, wall_sprite) in wall_query.iter() {
                    if let Some(_) = collide(
                        x_pos,
                        sprite.custom_size.unwrap_or(Vec2::ONE),
                        wall_transform.translation,
                        wall_sprite.custom_size.unwrap_or(Vec2::ONE),
                    ) {
                        x_blocked = true;
                        break;
                    }
                }
                if !x_blocked {
                    new_pos = x_pos;
                }
            }
            
            // Try Y movement
            if direction.y != 0.0 {
                let y_pos = new_pos + Vec3::new(0.0, direction.y * delta, 0.0);
                let mut y_blocked = false;
                for (wall_transform, wall_sprite) in wall_query.iter() {
                    if let Some(_) = collide(
                        y_pos,
                        sprite.custom_size.unwrap_or(Vec2::ONE),
                        wall_transform.translation,
                        wall_sprite.custom_size.unwrap_or(Vec2::ONE),
                    ) {
                        y_blocked = true;
                        break;
                    }
                }
                if !y_blocked {
                    new_pos = y_pos;
                }
            }
            
            transform.translation = new_pos;
        }
    }
}

pub fn spawn_coins(
    mut commands: Commands,
    room_query: Query<(&Room, &Transform)>,
    game_assets: Res<GameAssets>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(COIN_SPAWN_CHANCE as f64) {
        for (room, _) in room_query.iter() {
            let random_position = Vec2::new(
                rng.gen_range(-ROOM_SIZE/2.0 + 50.0..ROOM_SIZE/2.0 - 50.0),
                rng.gen_range(-ROOM_SIZE/2.0 + 50.0..ROOM_SIZE/2.0 - 50.0),
            );

            commands.spawn((
                SpriteBundle {
                    texture: game_assets.coin_sprite.clone(),
                    sprite: Sprite {
                        custom_size: Some(COIN_SIZE),
                        color: Color::rgba(1.0, 1.0, 1.0, 0.5), // 50% opacity
                        ..default()
                    },
                    transform: Transform::from_translation(
                        (room.position + random_position).extend(1.0)
                    ),
                    ..default()
                },
                Coin {
                    velocity: Vec2::ZERO,
                    stationary_timer: Timer::new(Duration::from_secs(COIN_STATIONARY_TIME as u64), TimerMode::Once),
                },
                Cleanup,
            ));
        }
    }
}

pub fn handle_coin_state_change(
    mut coin_query: Query<(&mut Coin, &mut Sprite)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    
    for (mut coin, mut sprite) in coin_query.iter_mut() {
        if coin.stationary_timer.tick(time.delta()).just_finished() {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            coin.velocity = Vec2::new(angle.cos(), angle.sin()) * COIN_SPEED;
            sprite.color.set_a(1.0); // Full opacity when moving
        }
    }
}

pub fn coin_movement(
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

pub fn check_coin_collision(
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    coin_query: Query<(&Transform, &Sprite, &Coin), With<Coin>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
) {
    // Only check collisions if we're in Playing state
    if *game_state.get() == GameState::Playing {
        if let Ok((player_transform, player_sprite)) = player_query.get_single() {
            for (coin_transform, coin_sprite, coin) in coin_query.iter() {
                // Only check collision if coin is moving (not stationary)
                if coin.velocity != Vec2::ZERO {
                    if let Some(_) = collide(
                        player_transform.translation,
                        player_sprite.custom_size.unwrap_or(Vec2::ONE),
                        coin_transform.translation,
                        coin_sprite.custom_size.unwrap_or(Vec2::ONE),
                    ) {
                        next_state.set(GameState::GameOver);
                        return;
                    }
                }
            }
        }
    }
}

pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

pub fn check_room_generation(
    player_query: Query<&Transform, With<Player>>,
    room_query: Query<(&Room, &Transform)>,
    mut commands: Commands,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_pos = player_transform.translation.truncate();
        
        // Find the current room
        if let Some((current_room, _)) = room_query
            .iter()
            .find(|(room, _)| {
                let room_bounds = ROOM_SIZE / 2.0;
                let room_pos = room.position;
                player_pos.x >= room_pos.x - room_bounds
                    && player_pos.x <= room_pos.x + room_bounds
                    && player_pos.y >= room_pos.y - room_bounds
                    && player_pos.y <= room_pos.y + room_bounds
            })
        {
            // Generate rooms recursively up to ROOM_GENERATION_DISTANCE
            generate_rooms_recursive(&mut commands, current_room, &room_query, ROOM_GENERATION_DISTANCE);
        }
    }
}

fn generate_rooms_recursive(
    commands: &mut Commands,
    current_room: &Room,
    room_query: &Query<(&Room, &Transform)>,
    depth: i32,
) {
    if depth <= 0 {
        return;
    }

    for &exit in &current_room.exits {
        let next_room_pos = current_room.position + exit.to_vec2() * ROOM_SIZE;
        
        // Check if room already exists at this position
        if !room_query.iter().any(|(room, _)| room.position == next_room_pos) {
            // Generate random exits for the new room
            let mut rng = rand::thread_rng();
            let num_exits = rng.gen_range(1..=4);
            let mut available_exits = Direction::all();
            available_exits.retain(|&dir| dir != exit.opposite()); // Ensure connection to previous room
            available_exits.shuffle(&mut rng);
            let mut new_exits = vec![exit.opposite()]; // Always include connection to previous room
            new_exits.extend(available_exits.iter().take(num_exits - 1).cloned());
            
            let new_room = Room {
                position: next_room_pos,
                exits: new_exits.clone(),
            };
            
            spawn_room(commands, next_room_pos, new_exits);
            
            // Recursively generate rooms from the new room
            generate_rooms_recursive(commands, &new_room, room_query, depth - 1);
        }
    }
}

pub fn despawn_invisible_coins(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
) {
    // Get the camera and window
    let (_camera, camera_transform) = camera_query.single();
    let window = windows.single();

    // Calculate visible area in world coordinates
    let window_size = Vec2::new(window.width(), window.height());
    let visible_area_half = window_size / 2.0 + Vec2::splat(VISIBILITY_BUFFER);

    // Get camera position in world coordinates
    let camera_pos = camera_transform.translation().truncate();

    // Check each coin
    for (entity, transform) in coin_query.iter() {
        let coin_pos = transform.translation.truncate();
        let offset_from_camera = coin_pos - camera_pos;

        // If coin is outside visible area, despawn it
        if offset_from_camera.x.abs() > visible_area_half.x || 
           offset_from_camera.y.abs() > visible_area_half.y {
            commands.entity(entity).despawn();
        }
    }
}

pub fn setup_ui(mut commands: Commands) {
    // Timer text in bottom left
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Time: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0.0",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..default()
        }),
        Cleanup,
    ));

    // Game over text and restart button (hidden initially)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(40.0),
                margin: UiRect {
                    left: Val::Px(-200.0), // Center the text by offsetting half its width
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText,
        Cleanup,
    )).with_children(|parent| {
        // Game Over text
        parent.spawn(TextBundle::from_sections([
            TextSection::new(
                "Game Over!\n",
                TextStyle {
                    font_size: 50.0,
                    color: Color::RED,
                    ..default()
                },
            ),
            TextSection::new(
                "You survived for ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                "0.0",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::new(
                " seconds",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
        ]));

        // Restart button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
                ..default()
            },
            ButtonAction::Restart,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Restart",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

pub fn update_timer(
    time: Res<Time>,
    mut game_timer: ResMut<GameTimer>,
    mut query: Query<&mut Text>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() == GameState::Playing {
        game_timer.elapsed_secs += time.delta_seconds();
        
        // Update timer text
        for mut text in query.iter_mut() {
            if text.sections.len() >= 2 {  // Make sure we have both sections
                text.sections[1].value = format!("{:.1}", game_timer.elapsed_secs);
            }
        }
    }
}

pub fn show_game_over(
    mut game_over_query: Query<(&mut Visibility, &Children), With<GameOverText>>,
    mut text_query: Query<&mut Text>,
    game_timer: Res<GameTimer>,
) {
    if let Ok((mut visibility, children)) = game_over_query.get_single_mut() {
        *visibility = Visibility::Visible;
        
        // Find and update the text with the final time
        for &child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(child) {
                if text.sections.len() >= 3 {
                    text.sections[2].value = format!("{:.1}", game_timer.elapsed_secs);
                    break;  // Exit after finding and updating the correct text
                }
            }
        }
    }
}

pub fn setup_menu(mut commands: Commands) {
    // UI Camera
    commands.spawn((
        Camera2dBundle::default(),
        MenuCleanup,
    ));

    // Root node
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.1, 0.1, 0.1)),
            ..default()
        },
        MenuUI,
        MenuCleanup,
    )).with_children(|parent| {
        // Title
        parent.spawn(TextBundle {
            text: Text::from_section(
                "DODGECOIN",
                TextStyle {
                    font_size: 80.0,
                    color: Color::rgb(1.0, 0.7, 0.3), // Light orange
                    ..default()
                },
            ),
            style: Style {
                margin: UiRect::all(Val::Px(50.0)),
                ..default()
            },
            ..default()
        });

        // Start Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
                ..default()
            },
            ButtonAction::Start,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });

        // Exit Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
                ..default()
            },
            ButtonAction::Exit,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Exit",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
    });
}

pub fn handle_buttons(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_timer: ResMut<GameTimer>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    ButtonAction::Start => {
                        next_state.set(GameState::Playing);
                    }
                    ButtonAction::Exit => {
                        exit.send(AppExit);
                    }
                    ButtonAction::Restart => {
                        // Reset game timer
                        game_timer.elapsed_secs = 0.0;
                        // Transition to Menu state (which will clean up everything)
                        next_state.set(GameState::Menu);
                        // Then immediately transition to Playing state
                        next_state.set(GameState::Playing);
                    }
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.4, 0.4, 0.4).into(); // Lighter grey when hovered
            }
            Interaction::None => {
                *color = Color::rgb(0.3, 0.3, 0.3).into(); // Default grey
            }
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MenuCleanup>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_system(
    mut commands: Commands,
    query: Query<Entity, With<Cleanup>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}