# DodgeCoin Game

A simple 2D game where you control a ball trying to dodge moving coins. Built with Rust and the Bevy game engine.

## Game Rules
- Control your player (blue square) using WASD keys
- Avoid the coins (yellow squares)
- Coins spawn randomly and stay stationary for 3 seconds
- After 3 seconds, coins start moving in random directions and bounce off walls
- If you get hit by a coin, the game is over

## Controls
- W: Move up
- S: Move down
- A: Move left
- D: Move right
- (You can combine keys for diagonal movement)

## Requirements
- Rust (latest stable version)
- Cargo (comes with Rust)

## How to Run
1. Clone this repository
2. Navigate to the project directory
3. Run the game:
```bash
cargo run --release
```

## Development
The game is built using:
- Bevy 0.12.0 - Game engine
- rand 0.8.5 - Random number generation

## Performance
The game includes some optimization settings in Cargo.toml for better performance in both debug and release modes.
