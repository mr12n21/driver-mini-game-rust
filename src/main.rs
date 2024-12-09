use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand, // Required for `.execute`
};
use rand::Rng;
use std::io::{stdout, Write};
use std::{thread, time::Duration};

const ROAD_WIDTH: usize = 20;
const SCREEN_HEIGHT: u16 = 20;
const PLAYER_SYMBOL: char = '^';
const OBSTACLE_SYMBOL: char = '|';

struct Game {
    player_x: usize,
    obstacles: Vec<(usize, u16)>, // x-coordinate and y-coordinate of obstacles
}

impl Game {
    fn new() -> Self {
        Self {
            player_x: ROAD_WIDTH / 2,
            obstacles: Vec::new(),
        }
    }

    fn handle_input(&mut self) {
        while crossterm::event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = crossterm::event::read().unwrap() {
                match key.code {
                    KeyCode::Left if self.player_x > 0 => self.player_x -= 1,
                    KeyCode::Right if self.player_x < ROAD_WIDTH - 1 => self.player_x += 1,
                    KeyCode::Esc => std::process::exit(0),
                    _ => {}
                }
            }
        }
    }

    fn update(&mut self) {
        // Randomly generate obstacles
        if rand::thread_rng().gen_range(0..5) == 0 {
            self.obstacles.push((rand::thread_rng().gen_range(0..ROAD_WIDTH), 0));
        }

        // Move obstacles downward
        for obstacle in &mut self.obstacles {
            obstacle.1 += 1;
        }

        // Remove obstacles that are out of the screen
        self.obstacles.retain(|&(_, y)| y < SCREEN_HEIGHT);
    }

    fn check_collision(&self) -> bool {
        for &(obstacle_x, obstacle_y) in &self.obstacles {
            if obstacle_y == SCREEN_HEIGHT - 1 && obstacle_x == self.player_x {
                return true; // Collision detected
            }
        }
        false
    }

    fn render(&self) {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        stdout.execute(crossterm::cursor::MoveTo(0, 0)).unwrap();

        // Render player
        stdout.execute(crossterm::cursor::MoveTo(self.player_x as u16, SCREEN_HEIGHT - 1))
            .unwrap();
        write!(stdout, "{}", PLAYER_SYMBOL).unwrap();

        // Render obstacles
        for &(obstacle_x, obstacle_y) in &self.obstacles {
            if obstacle_y < SCREEN_HEIGHT {
                stdout.execute(crossterm::cursor::MoveTo(obstacle_x as u16, obstacle_y))
                    .unwrap();
                write!(stdout, "{}", OBSTACLE_SYMBOL).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(crossterm::cursor::Hide).unwrap();

    let mut game = Game::new();

    loop {
        game.handle_input();
        game.update();
        game.render();
        if game.check_collision() {
            println!("\nGame Over!");
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }

    terminal::disable_raw_mode().unwrap();
    stdout.execute(crossterm::cursor::Show).unwrap();
}
