use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Component, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point);


#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Debug, Component, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}
