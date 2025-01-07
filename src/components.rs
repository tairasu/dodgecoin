use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Coin {
    pub velocity: Vec2,
    pub stationary_timer: Timer,
}

#[derive(Component)]
pub struct Room {
    pub position: Vec2,
    pub exits: Vec<Direction>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn to_vec2(&self) -> Vec2 {
        match self {
            Direction::North => Vec2::new(0.0, 1.0),
            Direction::South => Vec2::new(0.0, -1.0),
            Direction::East => Vec2::new(1.0, 0.0),
            Direction::West => Vec2::new(-1.0, 0.0),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub enum ButtonAction {
    Start,
    Exit,
    Restart,
}

#[derive(Component)]
pub struct MenuCleanup;

#[derive(Component)]
pub struct Cleanup; 