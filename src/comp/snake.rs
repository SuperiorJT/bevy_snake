use bevy::prelude::*;
use std::collections::LinkedList;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Snake {
    pub body: LinkedList<Entity>,
    pub direction: SnakeDirection,
    pub last_direction: SnakeDirection,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            body: LinkedList::new(),
            direction: SnakeDirection::Up,
            last_direction: SnakeDirection::Up
        }
    }
}

#[derive(Debug, Copy, Clone, Property, Serialize, Deserialize)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeHead;
pub struct SnakeTail;
pub struct SnakeBody;