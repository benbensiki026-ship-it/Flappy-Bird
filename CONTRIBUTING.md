# Contributing to Flappy Bird - Rust Edition

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** (if hosted on a platform like GitHub)
2. **Clone your fork**:
   ```bash
   git clone https://github.com/yourusername/flappy_bird.git
   cd flappy_bird
   ```
3. **Create a branch** for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites
- Rust 1.70.0 or later
- Cargo (included with Rust)
- A text editor or IDE (VS Code, IntelliJ IDEA, etc.)

### Building the Project
```bash
# Quick check (fast)
cargo check

# Build debug version
cargo build

# Build and run
cargo run

# Build release version
cargo build --release
```

## Code Style

### Rust Style Guidelines
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Run `cargo fmt` before committing to format code
- Run `cargo clippy` to catch common mistakes and improve code
- Keep functions small and focused
- Add comments for complex logic
- Use meaningful variable and function names

### Code Organization
```
src/
└── main.rs         # All game code (currently monolithic)
                    # Consider splitting into modules for larger changes:
                    # - bird.rs (Bird struct and impl)
                    # - pipe.rs (Pipe struct and impl)
                    # - particle.rs (Particle effects)
                    # - game.rs (Game state management)
                    # - ui.rs (Drawing functions)
```

## Testing

### Running Tests
```bash
cargo test
```

### Writing Tests
- Add unit tests in the same file as the code being tested
- Use integration tests in `tests/` directory for end-to-end testing
- Example:
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_bird_jump() {
          let mut bird = Bird::new(100.0, 100.0);
          bird.jump();
          assert!(bird.velocity < 0.0);
      }
  }
  ```

## Feature Ideas

### Easy Additions
- [ ] Sound effects (jump, score, collision)
- [ ] Background music
- [ ] More particle effects
- [ ] Additional bird skins/colors
- [ ] Day/night cycle
- [ ] Weather effects (rain, snow)

### Medium Complexity
- [ ] Power-ups (shield, slow-mo, extra life)
- [ ] Multiple bird types with different abilities
- [ ] Achievements system
- [ ] Combo scoring (consecutive successful passes)
- [ ] Online leaderboards
- [ ] Mobile controls (touch/tilt)

### Advanced Features
- [ ] Multiplayer mode
- [ ] Level editor
- [ ] Procedural pipe generation patterns
- [ ] Replay system
- [ ] Statistics tracking
- [ ] AI bird that learns to play

## Pull Request Process

1. **Update documentation** if you're adding features or changing behavior
2. **Test your changes** thoroughly
3. **Run formatting and linting**:
   ```bash
   cargo fmt
   cargo clippy
   ```
4. **Write clear commit messages**:
   ```
   Add particle effects for score gains
   
   - Created new particle type for celebrations
   - Added spawn_particles method to Game struct
   - Particles now trigger on each score increment
   ```
5. **Submit the pull request** with:
   - Clear description of changes
   - Screenshots/GIFs for visual changes
   - Testing steps
   - Related issue numbers

## Bug Reports

When reporting bugs, please include:
- **Description**: What happened vs what you expected
- **Steps to reproduce**: Detailed steps to recreate the bug
- **Environment**: OS, Rust version, game version
- **Screenshots/videos**: If applicable
- **Error messages**: Full error output if the game crashes

### Bug Report Template
```markdown
**Bug Description**
A clear description of the bug

**To Reproduce**
1. Start game
2. Select Hard difficulty
3. Score 10 points
4. Bug occurs

**Expected Behavior**
What should happen

**Screenshots**
Add screenshots here

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Game version: [e.g., 0.1.0]

**Additional Context**
Any other relevant information
```

## Code of Conduct

### Our Standards
- Be respectful and inclusive
- Accept constructive criticism gracefully
- Focus on what's best for the project
- Show empathy towards other contributors

### Unacceptable Behavior
- Harassment or discriminatory language
- Trolling or insulting comments
- Publishing others' private information
- Other unprofessional conduct

## Questions?

- Open an issue with the "question" label
- Reach out to maintainers
- Check existing issues and pull requests first

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing! 🎮🐦
