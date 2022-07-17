extern crate rand;
extern crate tcod;

use map::map::Map;
use rand::Rng;
use std::boxed::Box;
use tcod::colors::{self, Color};
use tcod::console::*;
//use entities::movement;
//use entities::movement::player_movement::PlayerMovement;

//mod entities;

pub struct Object {
    pub x: i32,
    pub y: i32,
    char: char,
    color: Color,
    //input: &'a PlayerMovement,
}

impl<'a> Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
            //input: input,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !map.map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }

    pub fn act(&mut self, map: &Map) {
        let direction = rand::thread_rng().gen_range(0, 3);
        match direction {
            0 => self.move_by(0, 1, map),
            2 => self.move_by(0, -1, map),
            3 => self.move_by(1, 0, map),
            4 => self.move_by(-1, 0, map),
            _ => {}
        };
    }
}
