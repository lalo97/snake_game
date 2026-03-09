use piston_window::graphics::types::Color;
use piston_window::graphics::{Context, Graphics};
use piston_window::*;

use rand::random_range;

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

/// Top-level game state: snake, food, board size, and timing.
pub struct Game {
    snake: Snake,

    food: Option<(i32, i32)>,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    /// Initializes a new game with a fresh snake and food at a fixed starting position.
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food: Some((6, 4)),
            width,
            height,
            game_over: false,
        }
    }

    /// Handles a keyboard event; updates the snake's direction if the key is an arrow key.
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(dir) = dir {
            if dir != self.snake.head_direction().opposite() {
                self.update_snake(Some(dir));
            }
        }
    }

    /// Renders the snake, food, border, and game-over overlay.
    pub fn draw<G: Graphics>(&self, con: &Context, g: &mut G) {
        self.snake.draw(con, g);

        if let Some((food_x, food_y)) = self.food {
            draw_block(FOOD_COLOR, food_x, food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    /// Advances game state by one frame; drives movement timing and restart logic.
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if self.food.is_none() {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// Checks if the snake's head is on the food; if so, grows the snake and removes the food.
    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food == Some((head_x, head_y)) {
            self.food = None;
            self.snake.restore_tail();
        }
    }

    /// Returns `true` if moving in `dir` would keep the snake within bounds and collision-free.
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        !self.snake.overlap_tail(next_x, next_y)
            && next_x > 0
            && next_y > 0
            && next_x < self.width - 1
            && next_y < self.height - 1
    }

    /// Randomly places food on an unoccupied cell.
    fn add_food(&mut self) {
        let (new_x, new_y) = loop {
            let x = random_range(1..self.width - 1);
            let y = random_range(1..self.height - 1);
            if !self.snake.overlap_tail(x, y) {
                break (x, y);
            }
        };
        self.food = Some((new_x, new_y));
    }

    /// Moves the snake (or sets game_over) and resets the frame timer.
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// Resets the entire game to its initial state.
    fn restart(&mut self) {
        *self = Game::new(self.width, self.height);
    }
}
