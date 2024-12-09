use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::Color,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::{thread, time::Duration};

const WIDTH: usize = 50;
const HEIGHT: usize = 20;
const ROAD_WIDTH: usize = 12;

struct Car {
    position: usize,
    velocity: f64, // Simulate realistic velocity
}

impl Car {
    fn new() -> Self {
        Self {
            position: WIDTH / 2,
            velocity: 0.0,
        }
    }

    /// Update the car's position with physics
    fn update(&mut self) {
        self.position = (self.position as f64 + self.velocity) as usize;

        // Clamp the position so it doesn't go out of bounds
        if self.position < 0 {
            self.position = 0;
        } else if self.position >= WIDTH {
            self.position = WIDTH - 1;
        }
    }

    /// Apply left/right movement changes
    fn apply_input(&mut self, input: f64) {
        self.velocity = input;
    }
}

struct Game {
    car: Car,
    road: VecDeque<char>, // Road as a scrolling 1D representation
    game_over: bool,
    score: usize,
}

impl Game {
    fn new() -> Self {
        let mut road = VecDeque::new();
        for _ in 0..HEIGHT {
            road.push_back(random_road_segment());
        }

        Self {
            car: Car::new(),
            road,
            game_over: false,
            score: 0,
        }
    }

    /// Scroll the "road" effect by moving the deque forward
    fn scroll_road(&mut self) {
        self.road.pop_front();
        self.road.push_back(random_road_segment());
    }

    /// Render the scene on screen
    fn render(&self) {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        for y in 0..HEIGHT {
            stdout.execute(crossterm::style::SetForegroundColor(Color::White)).unwrap();
            write!(stdout, "{}", self.road[y]);
            stdout.execute(crossterm::style::ResetColor).unwrap();
            writeln!(stdout).unwrap();
        }

        // Render the car
        stdout.execute(crossterm::style::SetForegroundColor(Color::Yellow)).unwrap();
        write!(stdout, "{}", "^".repeat(1));
        stdout.execute(crossterm::style::ResetColor).unwrap();
        writeln!(stdout).unwrap();
        stdout.flush().unwrap();
    }

    /// Handle user input (left/right/stop/quit logic)
    fn handle_input(&mut self) {
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Left => self.car.apply_input(-1.0),
                    KeyCode::Right => self.car.apply_input(1.0),
                    KeyCode::Esc => self.game_over = true,
                    _ => {}
                }
            }
        }
    }

    fn main_loop(&mut self) {
        while !self.game_over {
            self.handle_input();
            self.car.update();
            self.scroll_road();
            self.render();
            self.score += 1;

            thread::sleep(Duration::from_millis(33));
        }

        println!("Game Over! Final Score: {}", self.score);
    }
}

/// Simulate road randomness
fn random_road_segment() -> char {
    if rand::random::<f32>() > 0.8 {
        '.'
    } else {
        ' '
    }
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let mut game = Game::new();
    game.main_loop();

    terminal::disable_raw_mode().unwrap();
    stdout.execute(cursor::Show).unwrap();
}
