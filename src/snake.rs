use std::collections::VecDeque;

use piston_window::graphics::{Context, Graphics, types::Color};

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.0, 0.80, 0.0, 1.0];

/// The four cardinal movement directions.
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the direction directly opposite to `self`.
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    /// Returns the `(dx, dy)` grid delta for one step in this direction.
    fn offset(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}

/// A single grid cell occupied by part of the snake.
#[derive(Clone, Copy, Debug)]
struct Block {
    x: i32,
    y: i32,
}

/// The player-controlled snake: body, direction, and saved tail.
pub struct Snake {
    direction: Direction,
    body: VecDeque<Block>,
    tail: Option<Block>,
}

impl Snake {
    /// Creates a 3-block snake with its head at `(x+2, y)`, moving right.
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body = VecDeque::new();

        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    /// Renders every body segment to the screen.
    pub fn draw<G: Graphics>(&self, con: &Context, g: &mut G) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    /// Returns the `(x, y)` grid coordinates of the snake's head.
    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    /// Returns the current movement direction of the snake.
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    /// Advances the snake one step; optionally changes direction first.
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d;
        }

        let (head_x, head_y) = self.head_position();
        let (dx, dy) = self.direction.offset();

        self.body.push_front(Block { x: head_x + dx, y: head_y + dy });
        self.tail = self.body.pop_back();
    }

    /// Returns the grid position the head would occupy after one step.
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let (dx, dy) = dir.unwrap_or(self.direction).offset();
        (head_x + dx, head_y + dy)
    }

    /// Re-attaches the previously removed tail segment (used when eating).
    pub fn restore_tail(&mut self) {
        self.body.push_back(self.tail.unwrap());
    }

    /// Returns `true` if `(x, y)` overlaps any body segment except the current tail.
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let check_len = self.body.len().saturating_sub(1);
        self.body.iter().take(check_len).any(|b| b.x == x && b.y == y)
    }
}
