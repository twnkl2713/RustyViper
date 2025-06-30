use piston_window::{Context, G2d, Glyphs, Key};
use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle, draw_game_ui, draw_centered_text};

const FOOD_COLOR: [f32; 4] = [0.80, 0.00, 0.00, 1.0];
const BONUS_FOOD_COLOR: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
const BORDER_COLOR: [f32; 4] = [0.10, 0.10, 0.10, 1.0];
const GAMEOVER_COLOR: [f32; 4] = [0.90, 0.00, 0.00, 0.5];

const BASE_SPEED: f64 = 0.15;
const POINTS_PER_LEVEL: usize = 5;

#[derive(PartialEq)]
enum GameState {
    Running,
    Paused,
    GameOver,
}

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    bonus_food: bool,
    width: i32,
    height: i32,
    score: usize,
    level: usize,
    waiting_time: f64,
    game_state: GameState,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: 6,
            food_y: 4,
            bonus_food: false,
            width,
            height,
            score: 0,
            level: 1,
            game_state: GameState::Running,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::P => {
                self.game_state = if self.game_state == GameState::Running {
                    GameState::Paused
                } else {
                    GameState::Running
                };
            }
            Key::R => self.restart(),
            _ => {
                if self.game_state == GameState::Running {
                    let dir = match key {
                        Key::Up => Some(Direction::Up),
                        Key::Down => Some(Direction::Down),
                        Key::Left => Some(Direction::Left),
                        Key::Right => Some(Direction::Right),
                        _ => None,
                    };

                    if let Some(d) = dir {
                        if d != self.snake.head_direction().opposite() {
                            self.update_snake(Some(d));
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut Glyphs) {
        self.snake.draw(con, g);

        if self.food_exists {
            let color = if self.bonus_food {
                BONUS_FOOD_COLOR
            } else {
                FOOD_COLOR
            };
            draw_block(color, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_state == GameState::GameOver {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);

            let center_x = (self.width as f64 * 25.0) / 2.0;
            let screen_mid_y = (self.height as f64 * 25.0) / 2.0;

            draw_centered_text(
                "💀 Game Over 💀",
                36,
                [1.0, 0.2, 0.2, 1.0],
                center_x,
                screen_mid_y - 20.0,
                con,
                g,
                glyphs,
            );

            draw_centered_text(
                "Press R to Restart",
                24,
                [1.0, 1.0, 1.0, 1.0],
                center_x,
                screen_mid_y + 20.0,
                con,
                g,
                glyphs,
            );
        }
    
        draw_game_ui(self.score, self.level, con, g, glyphs);
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.game_state != GameState::Running {
            return;
        }

        self.waiting_time += delta_time;

        if !self.food_exists {
            self.add_food();
        }

        let speed = (BASE_SPEED - (self.level as f64 * 0.01)).max(0.05);

        if self.waiting_time > speed {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && head_x == self.food_x && head_y == self.food_y {
            self.food_exists = false;
            self.snake.restore_tail();

            self.score += if self.bonus_food { 3 } else { 1 };
            self.bonus_food = false;

            if self.score > 0 && self.score % POINTS_PER_LEVEL == 0 {
                self.level += 1;
            }
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        !self.snake.overlap_tail(next_x, next_y)
            && next_x > 0
            && next_y > 0
            && next_x < self.width - 1
            && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let (mut new_x, mut new_y);

        loop {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
            if !self.snake.overlap_tail(new_x, new_y) {
                break;
            }
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
        self.bonus_food = rng.gen_bool(0.2);
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_state = GameState::GameOver;
        }
        self.waiting_time = 0.0;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.bonus_food = false;
        self.score = 0;
        self.level = 1;
        self.game_state = GameState::Running;
    }
}
