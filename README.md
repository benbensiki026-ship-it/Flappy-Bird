# Flappy Bird - Rust Edition 🐦

A feature-rich implementation of Flappy Bird written in Rust using the Macroquad game engine.

## Features

### 🎮 Core Gameplay
- **Smooth Physics**: Realistic gravity and jump mechanics
- **Collision Detection**: Precise hitbox-based collision system
- **Scoring System**: Track your score as you navigate through pipes
- **Animated Graphics**: Beautiful bird animations, clouds, and scrolling background

### 🎯 Multiple Difficulty Levels
- **Easy**: Larger gaps, slower pipes (Gap: 220px, Speed: 2.0)
- **Medium**: Balanced challenge (Gap: 180px, Speed: 2.5)
- **Hard**: Smaller gaps, faster pipes (Gap: 140px, Speed: 3.0)
- **Extreme**: Ultimate challenge (Gap: 120px, Speed: 3.8)

### 🏆 Persistence
- **High Score Tracking**: Separate high scores for each difficulty level
- **JSON Storage**: Scores saved locally in `highscores.json`
- **Automatic Saving**: High scores automatically persist between sessions

### ✨ Visual Effects
- **Particle System**: Explosion effects on collisions and score gains
- **Color Animations**: Dynamic visual feedback
- **Smooth Animations**: Bird rotation based on velocity
- **Parallax Background**: Scrolling clouds and background

### 🎨 Game States
- **Main Menu**: Choose difficulty and view high scores
- **Playing**: Active gameplay
- **Paused**: Pause and resume anytime
- **Game Over**: View final score and retry

### 🛠️ Debug Features
- **Hitbox Visualization**: Press `H` to toggle collision boxes
- **Invincibility Mode**: Press `I` to toggle god mode
- **Slow Motion**: Press `S` to slow down time
- **Performance Optimized**: Efficient rendering and updates

## Controls

### Main Menu
- `SPACE` or `ENTER` - Start game
- `1` - Select Easy difficulty
- `2` - Select Medium difficulty
- `3` - Select Hard difficulty
- `4` - Select Extreme difficulty

### In-Game
- `SPACE` or `LEFT CLICK` - Jump
- `ESC` - Pause/Resume game
- `H` - Toggle hitbox display (debug)
- `I` - Toggle invincibility (cheat)
- `S` - Toggle slow motion (cheat)

### Game Over / Paused
- `SPACE` - Retry
- `Q` or `ESC` - Return to main menu

## Building and Running

### Prerequisites
- Rust toolchain (1.70.0 or later)
- Cargo (comes with Rust)

### Installation

1. Install Rust if you haven't already:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone or extract the project:
```bash
cd flappy_bird
```

3. Build and run:
```bash
# Run in debug mode (faster compilation)
cargo run

# Build optimized release version
cargo build --release

# Run release version
cargo run --release
```

### Binary Location
After building with `cargo build --release`, the executable will be located at:
- Linux/macOS: `target/release/flappy_bird`
- Windows: `target\release\flappy_bird.exe`

## Project Structure

```
flappy_bird/
├── Cargo.toml          # Project configuration and dependencies
├── .gitignore          # Git ignore rules
├── README.md           # This file
├── src/
│   └── main.rs         # Main game source code
└── target/             # Build output (gitignored)
```

## Dependencies

- **macroquad** (0.4): Cross-platform game engine
- **rand** (0.8): Random number generation for pipes
- **serde** (1.0): Serialization framework
- **serde_json** (1.0): JSON support for high scores

## Technical Details

### Game Constants
- Gravity: 0.5 pixels/frame²
- Jump Strength: -8.0 pixels/frame
- Bird Size: 30x30 pixels
- Pipe Width: 60 pixels
- Ground Height: 80 pixels

### Performance
- Target: 60 FPS
- Optimized collision detection
- Efficient particle system
- Memory-safe Rust implementation

## High Score Storage

High scores are automatically saved to `highscores.json` in the game directory:

```json
{
  "easy": 42,
  "medium": 28,
  "hard": 15,
  "extreme": 7
}
```

## Development

### Code Structure
- **Bird struct**: Player character with physics
- **Pipe struct**: Obstacle generation and collision
- **Particle struct**: Visual effects system
- **Game struct**: Main game state and logic
- **HighScores**: Persistent score management

### Adding Features
The code is well-structured and documented, making it easy to:
- Add new difficulty levels
- Implement power-ups
- Create new particle effects
- Customize visuals
- Add sound effects

## Building for Distribution

### Create optimized binary:
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

### Strip binary (reduce size):
```bash
strip target/release/flappy_bird
```

### Package for distribution:
```bash
# The project includes a proper .gitignore
# The zip command will exclude build artifacts
zip -r flappy_bird.zip flappy_bird/ -x "flappy_bird/target/*" "flappy_bird/.git/*"
```

## License

MIT License - Feel free to modify and distribute!

## Credits

Created as a Rust learning project showcasing:
- Game development in Rust
- State management
- Physics simulation
- File I/O and persistence
- Event handling
- Rendering and graphics

## Tips for High Scores

1. **Stay Centered**: Keep your bird in the middle of the screen when possible
2. **Steady Taps**: Use gentle, consistent jumps rather than panicked mashing
3. **Look Ahead**: Watch for upcoming pipes, not just the current one
4. **Practice Patience**: Start with Easy mode to learn the physics
5. **Use Rhythm**: Find a tapping rhythm that works for each difficulty

Enjoy the game! 🎮🐦
