use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(new_dir) = dir {
            self.direction = new_dir;
        }

        let (x, y) = self.head_position();
        let new_head = match self.direction {
            Direction::Up => Block { x, y: y - 1 },
            Direction::Down => Block { x, y: y + 1 },
            Direction::Left => Block { x: x - 1, y },
            Direction::Right => Block { x: x + 1, y },
        };

        self.body.push_front(new_head);
        self.tail = self.body.pop_back();
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (x, y) = self.head_position();
        let d = dir.unwrap_or(self.direction);

        match d {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }

    pub fn restore_tail(&mut self) {
        if let Some(tail_block) = &self.tail {
            self.body.push_back(tail_block.clone());
        }
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut iter = self.body.iter();
        iter.next(); // Skip head
        for block in iter {
            if block.x == x && block.y == y {
                return true;
            }
        }
        false
    }
}
