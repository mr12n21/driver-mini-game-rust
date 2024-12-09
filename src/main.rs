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
use std::thread;
use std::time::Duration;

const WIDTH: usize = 50;
const HEIGHT: usize = 30;
const ROAD_WIDTH: usize = 10;

struct Scene {
    road: VecDeque<char>,
    car_position: usize,
    car_velocity: isize,
    game_running: bool,
    score: usize,
}

impl Scene {
    fn new() -> Self {
        let road = VecDeque::from(vec![' '; WIDTH]);
        Self {
            road,
            car_position: WIDTH / 2,
            car_velocity: 0,
            game_running: true,
            score: 0,
        }
    }

    fn initialize_scene(&mut self) {
        for i in 0..WIDTH {
            if i >= (WIDTH - ROAD_WIDTH) / 2 && i < (WIDTH + ROAD_WIDTH) / 2 {
                self.road[i] = ' ';
            } else {
                self.road[i] = '.';
            }
        }
    }

    fn render(&self) {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(ClearType::All)).unwrap();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if col == self.car_position && row == HEIGHT / 2 {
                    stdout
                        .execute(crossterm::style::SetForegroundColor(Color::Yellow))
                        .unwrap();
                    write!(stdout, "^").unwrap();
                    stdout.execute(crossterm::style::ResetColor).unwrap();
                } else {
                    write!(stdout, "{}", self.road[col]).unwrap();
                }
            }
            writeln!(stdout).unwrap();
        }

        writeln!(stdout, "Score: {}", self.score).unwrap();
        stdout.flush().unwrap();
    }

    fn handle_input(&mut self) {
        while event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Left => self.car_velocity = -1,
                    KeyCode::Right => self.car_velocity = 1,
                    KeyCode::Up | KeyCode::Down => self.car_velocity = 0,
                    KeyCode::Esc => {
                        self.game_running = false;
                    }
                    _ => {}
                }
            }
        }
    }

    fn update(&mut self) {
        let new_position = self.car_position as isize + self.car_velocity;
        if new_position >= 0 && new_position < WIDTH as isize {
            self.car_position = new_position as usize;
        }

        if self.road[self.car_position] != ' ' {
            self.game_running = false;
        }

        self.score += 1;
    }
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let mut game_scene = Scene::new();
    game_scene.initialize_scene();

    while game_scene.game_running {
        game_scene.handle_input();
        game_scene.update();
        game_scene.render();
        thread::sleep(Duration::from_millis(33));
    }

    terminal::disable_raw_mode().unwrap();
    stdout.execute(cursor::Show).unwrap();
    println!("Game Over! Final Score: {}", game_scene.score);
}
