use macroquad::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;

const GRAVITY: f32 = 0.5;
const JUMP_STRENGTH: f32 = -8.0;
const BIRD_SIZE: f32 = 30.0;
const PIPE_WIDTH: f32 = 60.0;
const PIPE_GAP: f32 = 180.0;
const PIPE_SPEED: f32 = 2.5;
const GROUND_HEIGHT: f32 = 80.0;

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

impl Difficulty {
    fn pipe_gap(&self) -> f32 {
        match self {
            Difficulty::Easy => 220.0,
            Difficulty::Medium => 180.0,
            Difficulty::Hard => 140.0,
            Difficulty::Extreme => 120.0,
        }
    }

    fn pipe_speed(&self) -> f32 {
        match self {
            Difficulty::Easy => 2.0,
            Difficulty::Medium => 2.5,
            Difficulty::Hard => 3.0,
            Difficulty::Extreme => 3.8,
        }
    }

    fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Extreme => "Extreme",
        }
    }
}

struct Bird {
    x: f32,
    y: f32,
    velocity: f32,
    rotation: f32,
    color: Color,
}

impl Bird {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
            rotation: 0.0,
            color: YELLOW,
        }
    }

    fn update(&mut self) {
        self.velocity += GRAVITY;
        self.y += self.velocity;
        
        // Update rotation based on velocity
        self.rotation = (self.velocity * 3.0).clamp(-30.0, 90.0);
    }

    fn jump(&mut self) {
        self.velocity = JUMP_STRENGTH;
    }

    fn draw(&self) {
        let bird_rect = Rect::new(
            self.x - BIRD_SIZE / 2.0,
            self.y - BIRD_SIZE / 2.0,
            BIRD_SIZE,
            BIRD_SIZE,
        );

        // Draw bird body
        draw_circle(self.x, self.y, BIRD_SIZE / 2.0, self.color);
        
        // Draw eye
        draw_circle(self.x + 8.0, self.y - 5.0, 5.0, WHITE);
        draw_circle(self.x + 10.0, self.y - 5.0, 3.0, BLACK);
        
        // Draw beak
        draw_triangle(
            Vec2::new(self.x + BIRD_SIZE / 2.0, self.y),
            Vec2::new(self.x + BIRD_SIZE / 2.0 + 10.0, self.y - 5.0),
            Vec2::new(self.x + BIRD_SIZE / 2.0 + 10.0, self.y + 5.0),
            ORANGE,
        );
    }

    fn get_bounds(&self) -> Rect {
        Rect::new(
            self.x - BIRD_SIZE / 2.0 + 5.0,
            self.y - BIRD_SIZE / 2.0 + 5.0,
            BIRD_SIZE - 10.0,
            BIRD_SIZE - 10.0,
        )
    }
}

struct Pipe {
    x: f32,
    gap_y: f32,
    gap_height: f32,
    scored: bool,
    color_top: Color,
    color_bottom: Color,
}

impl Pipe {
    fn new(x: f32, gap_height: f32) -> Self {
        let mut rng = rand::thread_rng();
        let gap_y = rng.gen_range(150.0..(screen_height() - GROUND_HEIGHT - gap_height - 100.0));
        
        Self {
            x,
            gap_y,
            gap_height,
            scored: false,
            color_top: GREEN,
            color_bottom: GREEN,
        }
    }

    fn update(&mut self, speed: f32) {
        self.x -= speed;
    }

    fn draw(&self) {
        // Top pipe
        draw_rectangle(
            self.x,
            0.0,
            PIPE_WIDTH,
            self.gap_y,
            self.color_top,
        );
        draw_rectangle_lines(self.x, 0.0, PIPE_WIDTH, self.gap_y, 3.0, DARKGREEN);
        
        // Top pipe cap
        draw_rectangle(
            self.x - 5.0,
            self.gap_y - 20.0,
            PIPE_WIDTH + 10.0,
            20.0,
            self.color_top,
        );
        draw_rectangle_lines(
            self.x - 5.0,
            self.gap_y - 20.0,
            PIPE_WIDTH + 10.0,
            20.0,
            3.0,
            DARKGREEN,
        );

        // Bottom pipe
        let bottom_y = self.gap_y + self.gap_height;
        draw_rectangle(
            self.x,
            bottom_y + 20.0,
            PIPE_WIDTH,
            screen_height() - bottom_y - GROUND_HEIGHT - 20.0,
            self.color_bottom,
        );
        draw_rectangle_lines(
            self.x,
            bottom_y + 20.0,
            PIPE_WIDTH,
            screen_height() - bottom_y - GROUND_HEIGHT - 20.0,
            3.0,
            DARKGREEN,
        );
        
        // Bottom pipe cap
        draw_rectangle(
            self.x - 5.0,
            bottom_y,
            PIPE_WIDTH + 10.0,
            20.0,
            self.color_bottom,
        );
        draw_rectangle_lines(
            self.x - 5.0,
            bottom_y,
            PIPE_WIDTH + 10.0,
            20.0,
            3.0,
            DARKGREEN,
        );
    }

    fn collides_with(&self, bird: &Bird) -> bool {
        let bird_bounds = bird.get_bounds();
        
        // Check collision with top pipe
        let top_pipe = Rect::new(self.x, 0.0, PIPE_WIDTH, self.gap_y);
        if bird_bounds.overlaps(&top_pipe) {
            return true;
        }

        // Check collision with bottom pipe
        let bottom_y = self.gap_y + self.gap_height;
        let bottom_pipe = Rect::new(
            self.x,
            bottom_y,
            PIPE_WIDTH,
            screen_height() - bottom_y - GROUND_HEIGHT,
        );
        if bird_bounds.overlaps(&bottom_pipe) {
            return true;
        }

        false
    }

    fn is_offscreen(&self) -> bool {
        self.x + PIPE_WIDTH < 0.0
    }
}

#[derive(Serialize, Deserialize)]
struct HighScores {
    easy: i32,
    medium: i32,
    hard: i32,
    extreme: i32,
}

impl Default for HighScores {
    fn default() -> Self {
        Self {
            easy: 0,
            medium: 0,
            hard: 0,
            extreme: 0,
        }
    }
}

impl HighScores {
    fn load() -> Self {
        if let Ok(data) = fs::read_to_string("highscores.json") {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write("highscores.json", data);
        }
    }

    fn get(&self, difficulty: Difficulty) -> i32 {
        match difficulty {
            Difficulty::Easy => self.easy,
            Difficulty::Medium => self.medium,
            Difficulty::Hard => self.hard,
            Difficulty::Extreme => self.extreme,
        }
    }

    fn update(&mut self, difficulty: Difficulty, score: i32) -> bool {
        let current_high = self.get(difficulty);
        if score > current_high {
            match difficulty {
                Difficulty::Easy => self.easy = score,
                Difficulty::Medium => self.medium = score,
                Difficulty::Hard => self.hard = score,
                Difficulty::Extreme => self.extreme = score,
            }
            true
        } else {
            false
        }
    }
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color: Color,
    size: f32,
}

impl Particle {
    fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.vy += 0.2; // gravity
        self.life -= 0.02;
    }

    fn draw(&self) {
        let alpha = (self.life * 255.0) as u8;
        let color = Color::new(
            self.color.r,
            self.color.g,
            self.color.b,
            alpha as f32 / 255.0,
        );
        draw_circle(self.x, self.y, self.size, color);
    }

    fn is_dead(&self) -> bool {
        self.life <= 0.0
    }
}

struct Game {
    bird: Bird,
    pipes: Vec<Pipe>,
    particles: Vec<Particle>,
    score: i32,
    high_scores: HighScores,
    state: GameState,
    difficulty: Difficulty,
    pipe_spawn_timer: f32,
    background_offset: f32,
    show_hitboxes: bool,
    powerup_timer: f32,
    invincible: bool,
    slow_motion: bool,
    slow_motion_timer: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            bird: Bird::new(150.0, screen_height() / 2.0),
            pipes: Vec::new(),
            particles: Vec::new(),
            score: 0,
            high_scores: HighScores::load(),
            state: GameState::Menu,
            difficulty: Difficulty::Medium,
            pipe_spawn_timer: 0.0,
            background_offset: 0.0,
            show_hitboxes: false,
            powerup_timer: 0.0,
            invincible: false,
            slow_motion: false,
            slow_motion_timer: 0.0,
        }
    }

    fn reset(&mut self) {
        self.bird = Bird::new(150.0, screen_height() / 2.0);
        self.pipes.clear();
        self.particles.clear();
        self.score = 0;
        self.pipe_spawn_timer = 0.0;
        self.invincible = false;
        self.slow_motion = false;
        self.slow_motion_timer = 0.0;
    }

    fn spawn_pipe(&mut self) {
        let x = screen_width() + 50.0;
        self.pipes.push(Pipe::new(x, self.difficulty.pipe_gap()));
    }

    fn spawn_particles(&mut self, x: f32, y: f32, color: Color, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            self.particles.push(Particle {
                x,
                y,
                vx: rng.gen_range(-3.0..3.0),
                vy: rng.gen_range(-5.0..-1.0),
                life: 1.0,
                color,
                size: rng.gen_range(2.0..6.0),
            });
        }
    }

    fn update(&mut self) {
        match self.state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    self.reset();
                    self.state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Key1) {
                    self.difficulty = Difficulty::Easy;
                }
                if is_key_pressed(KeyCode::Key2) {
                    self.difficulty = Difficulty::Medium;
                }
                if is_key_pressed(KeyCode::Key3) {
                    self.difficulty = Difficulty::Hard;
                }
                if is_key_pressed(KeyCode::Key4) {
                    self.difficulty = Difficulty::Extreme;
                }
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Paused;
                    return;
                }

                // Handle jump
                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.bird.jump();
                    self.spawn_particles(self.bird.x, self.bird.y, SKYBLUE, 5);
                }

                // Toggle hitboxes (debug)
                if is_key_pressed(KeyCode::H) {
                    self.show_hitboxes = !self.show_hitboxes;
                }

                // Cheat codes for fun
                if is_key_pressed(KeyCode::I) {
                    self.invincible = !self.invincible;
                }
                if is_key_pressed(KeyCode::S) {
                    self.slow_motion = !self.slow_motion;
                }

                let time_scale = if self.slow_motion { 0.5 } else { 1.0 };

                // Update bird
                self.bird.update();

                // Update background
                self.background_offset -= 1.0 * time_scale;
                if self.background_offset <= -50.0 {
                    self.background_offset = 0.0;
                }

                // Spawn pipes
                self.pipe_spawn_timer += 1.0 * time_scale;
                if self.pipe_spawn_timer > 90.0 {
                    self.spawn_pipe();
                    self.pipe_spawn_timer = 0.0;
                }

                // Update pipes
                let speed = self.difficulty.pipe_speed() * time_scale;
                for pipe in &mut self.pipes {
                    pipe.update(speed);

                    // Check if bird passed pipe
                    if !pipe.scored && pipe.x + PIPE_WIDTH < self.bird.x {
                        pipe.scored = true;
                        self.score += 1;
                        self.spawn_particles(pipe.x + PIPE_WIDTH / 2.0, screen_height() / 2.0, GOLD, 15);
                    }

                    // Check collision
                    if !self.invincible && pipe.collides_with(&self.bird) {
                        self.state = GameState::GameOver;
                        self.spawn_particles(self.bird.x, self.bird.y, RED, 30);
                        
                        // Update high score
                        if self.high_scores.update(self.difficulty, self.score) {
                            self.high_scores.save();
                        }
                    }
                }

                // Remove offscreen pipes
                self.pipes.retain(|pipe| !pipe.is_offscreen());

                // Check ground/ceiling collision
                if !self.invincible && (self.bird.y - BIRD_SIZE / 2.0 <= 0.0 
                    || self.bird.y + BIRD_SIZE / 2.0 >= screen_height() - GROUND_HEIGHT) {
                    self.state = GameState::GameOver;
                    self.spawn_particles(self.bird.x, self.bird.y, RED, 30);
                    
                    if self.high_scores.update(self.difficulty, self.score) {
                        self.high_scores.save();
                    }
                }

                // Update particles
                for particle in &mut self.particles {
                    particle.update();
                }
                self.particles.retain(|p| !p.is_dead());
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Q) {
                    self.state = GameState::Menu;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    self.reset();
                    self.state = GameState::Playing;
                }
                if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
                    self.state = GameState::Menu;
                }
            }
        }
    }

    fn draw(&self) {
        // Draw animated background
        for i in 0..20 {
            let offset = (self.background_offset + i as f32 * 50.0) % screen_width();
            draw_rectangle(
                offset,
                0.0,
                50.0,
                screen_height() - GROUND_HEIGHT,
                Color::from_rgba(135 + (i % 3) as u8 * 10, 206, 235, 255),
            );
        }

        // Draw clouds
        for i in 0..5 {
            let x = (self.background_offset * 0.5 + i as f32 * 250.0) % (screen_width() + 100.0);
            draw_circle(x, 100.0 + i as f32 * 50.0, 40.0, WHITE);
            draw_circle(x + 30.0, 100.0 + i as f32 * 50.0, 50.0, WHITE);
            draw_circle(x + 60.0, 100.0 + i as f32 * 50.0, 40.0, WHITE);
        }

        match self.state {
            GameState::Menu => self.draw_menu(),
            GameState::Playing => self.draw_playing(),
            GameState::Paused => {
                self.draw_playing();
                self.draw_pause_overlay();
            }
            GameState::GameOver => {
                self.draw_playing();
                self.draw_game_over();
            }
        }
    }

    fn draw_menu(&self) {
        let title = "FLAPPY BIRD";
        let title_size = 80.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        
        draw_text(
            title,
            screen_width() / 2.0 - title_width / 2.0,
            200.0,
            title_size,
            YELLOW,
        );

        let instructions = vec![
            "Press SPACE or ENTER to Start",
            "",
            "Select Difficulty:",
            &format!("[1] Easy - High Score: {}", self.high_scores.easy),
            &format!("[2] Medium - High Score: {}", self.high_scores.medium),
            &format!("[3] Hard - High Score: {}", self.high_scores.hard),
            &format!("[4] Extreme - High Score: {}", self.high_scores.extreme),
            "",
            &format!("Current: {}", self.difficulty.name()),
            "",
            "Controls:",
            "SPACE / LEFT CLICK - Jump",
            "ESC - Pause",
            "H - Toggle Hitboxes (debug)",
            "I - Toggle Invincibility (cheat)",
            "S - Toggle Slow Motion (cheat)",
        ];

        let mut y = 300.0;
        for line in instructions {
            let size = if line.starts_with('[') || line.starts_with("Current:") {
                30.0
            } else if line.starts_with("Controls:") || line.starts_with("Select") {
                35.0
            } else {
                25.0
            };
            
            let color = if line.starts_with("Current:") {
                GOLD
            } else if line.contains("High Score") {
                GREEN
            } else {
                WHITE
            };

            let width = measure_text(line, None, size as u16, 1.0).width;
            draw_text(
                line,
                screen_width() / 2.0 - width / 2.0,
                y,
                size,
                color,
            );
            y += size + 10.0;
        }

        // Draw animated bird
        let bird_x = screen_width() / 2.0;
        let bird_y = 250.0 + (get_time() * 2.0).sin() as f32 * 10.0;
        draw_circle(bird_x, bird_y, BIRD_SIZE / 2.0, YELLOW);
        draw_circle(bird_x + 8.0, bird_y - 5.0, 5.0, WHITE);
        draw_circle(bird_x + 10.0, bird_y - 5.0, 3.0, BLACK);
    }

    fn draw_playing(&self) {
        // Draw pipes
        for pipe in &self.pipes {
            pipe.draw();
            
            if self.show_hitboxes {
                // Draw pipe hitboxes
                draw_rectangle_lines(pipe.x, 0.0, PIPE_WIDTH, pipe.gap_y, 2.0, RED);
                let bottom_y = pipe.gap_y + pipe.gap_height;
                draw_rectangle_lines(
                    pipe.x,
                    bottom_y,
                    PIPE_WIDTH,
                    screen_height() - bottom_y - GROUND_HEIGHT,
                    2.0,
                    RED,
                );
            }
        }

        // Draw particles
        for particle in &self.particles {
            particle.draw();
        }

        // Draw bird
        self.bird.draw();
        
        if self.show_hitboxes {
            let bounds = self.bird.get_bounds();
            draw_rectangle_lines(bounds.x, bounds.y, bounds.w, bounds.h, 2.0, RED);
        }

        // Draw ground
        draw_rectangle(
            0.0,
            screen_height() - GROUND_HEIGHT,
            screen_width(),
            GROUND_HEIGHT,
            Color::from_rgba(139, 69, 19, 255),
        );
        
        // Draw grass on ground
        for i in 0..((screen_width() / 20.0) as i32) {
            draw_rectangle(
                i as f32 * 20.0,
                screen_height() - GROUND_HEIGHT,
                20.0,
                10.0,
                Color::from_rgba(34, 139, 34, 255),
            );
        }

        // Draw score
        let score_text = format!("Score: {}", self.score);
        draw_text(&score_text, 20.0, 50.0, 40.0, WHITE);
        draw_text(&score_text, 18.0, 48.0, 40.0, BLACK);

        // Draw high score
        let high_score = self.high_scores.get(self.difficulty);
        let hs_text = format!("Best: {}", high_score);
        draw_text(&hs_text, 20.0, 90.0, 30.0, GOLD);

        // Draw difficulty
        let diff_text = format!("Difficulty: {}", self.difficulty.name());
        draw_text(&diff_text, screen_width() - 200.0, 50.0, 25.0, WHITE);

        // Draw status indicators
        if self.invincible {
            draw_text("INVINCIBLE", screen_width() / 2.0 - 80.0, 50.0, 30.0, GOLD);
        }
        if self.slow_motion {
            draw_text("SLOW MOTION", screen_width() / 2.0 - 90.0, 90.0, 30.0, SKYBLUE);
        }
    }

    fn draw_pause_overlay(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 180),
        );

        let pause_text = "PAUSED";
        let width = measure_text(pause_text, None, 80, 1.0).width;
        draw_text(
            pause_text,
            screen_width() / 2.0 - width / 2.0,
            screen_height() / 2.0 - 50.0,
            80.0,
            YELLOW,
        );

        let resume = "Press SPACE to Resume";
        let resume_width = measure_text(resume, None, 30, 1.0).width;
        draw_text(
            resume,
            screen_width() / 2.0 - resume_width / 2.0,
            screen_height() / 2.0 + 50.0,
            30.0,
            WHITE,
        );

        let quit = "Press Q for Main Menu";
        let quit_width = measure_text(quit, None, 25, 1.0).width;
        draw_text(
            quit,
            screen_width() / 2.0 - quit_width / 2.0,
            screen_height() / 2.0 + 100.0,
            25.0,
            WHITE,
        );
    }

    fn draw_game_over(&self) {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, 200),
        );

        let game_over = "GAME OVER";
        let width = measure_text(game_over, None, 80, 1.0).width;
        draw_text(
            game_over,
            screen_width() / 2.0 - width / 2.0,
            screen_height() / 2.0 - 100.0,
            80.0,
            RED,
        );

        let score_text = format!("Score: {}", self.score);
        let score_width = measure_text(&score_text, None, 40, 1.0).width;
        draw_text(
            &score_text,
            screen_width() / 2.0 - score_width / 2.0,
            screen_height() / 2.0 - 20.0,
            40.0,
            WHITE,
        );

        let high_score = self.high_scores.get(self.difficulty);
        let hs_text = if self.score > high_score {
            format!("NEW HIGH SCORE!")
        } else {
            format!("High Score: {}", high_score)
        };
        let hs_width = measure_text(&hs_text, None, 35, 1.0).width;
        let hs_color = if self.score > high_score { GOLD } else { YELLOW };
        draw_text(
            &hs_text,
            screen_width() / 2.0 - hs_width / 2.0,
            screen_height() / 2.0 + 30.0,
            35.0,
            hs_color,
        );

        let retry = "Press SPACE to Retry";
        let retry_width = measure_text(retry, None, 30, 1.0).width;
        draw_text(
            retry,
            screen_width() / 2.0 - retry_width / 2.0,
            screen_height() / 2.0 + 100.0,
            30.0,
            WHITE,
        );

        let menu = "Press Q for Main Menu";
        let menu_width = measure_text(menu, None, 25, 1.0).width;
        draw_text(
            menu,
            screen_width() / 2.0 - menu_width / 2.0,
            screen_height() / 2.0 + 150.0,
            25.0,
            WHITE,
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Flappy Bird - Rust Edition".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(SKYBLUE);
        
        game.update();
        game.draw();

        next_frame().await
    }
}
