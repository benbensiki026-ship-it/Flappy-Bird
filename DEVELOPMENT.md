# Development Guide

## Architecture Overview

### Game Loop
The game uses Macroquad's async game loop running at 60 FPS:
```
Initialize Game State
    ↓
Loop Forever:
    ├── Clear Background
    ├── Update Game State
    ├── Draw Frame
    └── Wait for Next Frame
```

### State Machine
```
Menu ←→ Playing ←→ Paused
         ↓
      GameOver → Menu
```

## Core Components

### 1. Bird Structure
```rust
struct Bird {
    x: f32,          // X position (constant)
    y: f32,          // Y position (changes)
    velocity: f32,   // Current velocity
    rotation: f32,   // Visual rotation
    color: Color,    // Bird color
}
```

**Physics Model**:
- Velocity increases by GRAVITY each frame (0.5 px/frame²)
- Jump sets velocity to JUMP_STRENGTH (-8.0 px/frame)
- Position updates by velocity each frame
- Rotation calculated from velocity for visual feedback

### 2. Pipe Structure
```rust
struct Pipe {
    x: f32,           // Horizontal position
    gap_y: f32,       // Top of gap
    gap_height: f32,  // Size of gap
    scored: bool,     // Has player passed?
    color_top: Color,
    color_bottom: Color,
}
```

**Generation**:
- Spawned off-screen (screen_width + 50)
- Gap positioned randomly with bounds checking
- Gap height varies by difficulty
- Removed when x < -PIPE_WIDTH

### 3. Particle System
```rust
struct Particle {
    x, y: f32,        // Position
    vx, vy: f32,      // Velocity
    life: f32,        // Remaining life (1.0 to 0.0)
    color: Color,
    size: f32,
}
```

**Behavior**:
- Spawned in bursts (5-30 particles)
- Affected by gravity (0.2 px/frame²)
- Alpha decreases with life
- Removed when life <= 0

### 4. Game State
```rust
struct Game {
    bird: Bird,
    pipes: Vec<Pipe>,
    particles: Vec<Particle>,
    score: i32,
    high_scores: HighScores,
    state: GameState,
    difficulty: Difficulty,
    // ... additional fields
}
```

## Collision Detection

### Algorithm
1. Get bird's bounding box (adjusted for visual sprite)
2. For each pipe:
   - Check overlap with top pipe rectangle
   - Check overlap with bottom pipe rectangle
3. Check ground collision (y + size/2 >= screen_height - GROUND_HEIGHT)
4. Check ceiling collision (y - size/2 <= 0)

### Optimization
- Early exit on first collision
- Only check visible pipes
- Hitboxes slightly smaller than visual sprites for forgiving gameplay

## Rendering Pipeline

### Layer Order (back to front)
1. **Background**: Animated sky gradient with clouds
2. **Pipes**: Green pipes with caps and borders
3. **Particles**: Explosion and celebration effects
4. **Bird**: Player character with eye and beak
5. **Ground**: Brown ground with grass
6. **UI**: Score, high score, status indicators

### Performance Considerations
- Particles culled when dead
- Pipes culled when off-screen
- Simple sprite drawing (no texture loading)
- Efficient rectangle drawing for backgrounds

## Difficulty Scaling

### Parameters by Difficulty
| Difficulty | Gap Size | Pipe Speed | Description |
|-----------|----------|------------|-------------|
| Easy      | 220px    | 2.0 px/f   | Learning mode |
| Medium    | 180px    | 2.5 px/f   | Standard challenge |
| Hard      | 140px    | 3.0 px/f   | Expert level |
| Extreme   | 120px    | 3.8 px/f   | Nearly impossible |

### Balancing Tips
- Gap size should allow 2-3 jumps to traverse
- Pipe speed affects reaction time needed
- Consider vertical travel distance per jump cycle

## High Score System

### Storage Format (JSON)
```json
{
  "easy": 42,
  "medium": 28,
  "hard": 15,
  "extreme": 7
}
```

### Implementation
- Loaded on game start
- Updated on game over if score beats previous
- Saved immediately after update
- File: `highscores.json` in working directory

### Error Handling
- Missing file creates default scores (all 0)
- Invalid JSON creates default scores
- Write failures are silent (non-critical)

## Input Handling

### Input Types
- **Keyboard**: Space, Enter, Escape, 1-4, H, I, S, Q
- **Mouse**: Left click for jump

### Input Processing
```
is_key_pressed() → Returns true once per press
is_mouse_button_pressed() → Returns true once per click
```

### State-Specific Inputs
- Menu: Start, difficulty selection
- Playing: Jump, pause, debug toggles
- Paused: Resume, quit
- GameOver: Retry, menu

## Adding New Features

### Example: Adding Sound Effects

1. **Add dependency** to `Cargo.toml`:
   ```toml
   [dependencies]
   macroquad = { version = "0.4", features = ["audio"] }
   ```

2. **Load audio** in main:
   ```rust
   let jump_sound = load_sound("assets/jump.wav").await.unwrap();
   ```

3. **Play on action**:
   ```rust
   if is_key_pressed(KeyCode::Space) {
       play_sound_once(jump_sound);
       self.bird.jump();
   }
   ```

### Example: Adding Power-ups

1. **Create power-up struct**:
   ```rust
   struct PowerUp {
       x: f32,
       y: f32,
       kind: PowerUpKind,
   }
   
   enum PowerUpKind {
       Shield,
       SlowMotion,
       ExtraLife,
   }
   ```

2. **Add to game state**:
   ```rust
   struct Game {
       // ... existing fields
       powerups: Vec<PowerUp>,
       active_powerup: Option<PowerUpKind>,
   }
   ```

3. **Implement spawn/collection logic**
4. **Update collision detection**
5. **Add visual effects**

## Performance Profiling

### Measuring Frame Time
```rust
let start = std::time::Instant::now();
// ... game update and draw
let frame_time = start.elapsed();
println!("Frame time: {:?}", frame_time);
```

### Common Bottlenecks
- Too many particles (limit spawn count)
- Excessive collision checks (spatial partitioning)
- Complex drawing operations (batch similar draws)

### Target Performance
- 60 FPS (16.67ms per frame)
- < 5ms update logic
- < 10ms rendering
- < 2ms remaining overhead

## Debugging Tips

### Enable Hitboxes
Press `H` during gameplay to visualize:
- Bird collision box (red outline)
- Pipe collision boxes (red outlines)

### Cheat Codes
- `I`: Toggle invincibility (pass through pipes)
- `S`: Toggle slow motion (0.5x speed)

### Logging
Add debug prints:
```rust
println!("Bird position: ({}, {})", self.bird.x, self.bird.y);
println!("Velocity: {}", self.bird.velocity);
println!("Pipe count: {}", self.pipes.len());
```

### Common Issues
1. **Bird falls too fast**: Increase JUMP_STRENGTH or decrease GRAVITY
2. **Pipes too hard**: Increase gap_height or decrease pipe_speed
3. **Collision feels unfair**: Adjust bird hitbox padding
4. **Performance issues**: Reduce particle count or check for memory leaks

## Testing Checklist

### Functional Tests
- [ ] Bird jumps on space/click
- [ ] Pipes spawn regularly
- [ ] Collision detection works
- [ ] Score increments correctly
- [ ] Game over triggers properly
- [ ] High scores save/load
- [ ] All difficulty levels work
- [ ] Pause/resume works
- [ ] Menu navigation works

### Edge Cases
- [ ] Rapid jumping at ceiling
- [ ] Staying at ground level
- [ ] No input (should fall to game over)
- [ ] Spam jumping
- [ ] Quick retry after game over
- [ ] Switching difficulties

### Platform Testing
- [ ] Linux (primary platform)
- [ ] Windows
- [ ] macOS
- [ ] Different screen resolutions

## Resources

### Macroquad Documentation
- [Main docs](https://docs.rs/macroquad/)
- [Examples](https://github.com/not-fl3/macroquad/tree/master/examples)
- [Book](https://macroquad.rs/articles/)

### Rust Game Dev
- [Are We Game Yet?](https://arewegameyet.rs/)
- [Rust Game Development subreddit](https://reddit.com/r/rust_gamedev)

### Game Design
- Physics tuning guides
- Difficulty balancing
- Juice and game feel articles

---

Happy coding! 🦀🎮
