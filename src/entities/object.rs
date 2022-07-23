extern crate rand;
extern crate tcod;

use map::map::Map;
//use rand::Rng;
use tcod::colors::Color;
use tcod::console::*;

pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
    pub name: String,
    blocks: bool,
    pub alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, name: &str, color: Color, blocks: bool) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
            name: name.into(),
            blocks,
            alive: false,
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con: &mut dyn Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }

    // pub fn act(&mut self, map: &Map) {
    //     let direction = rand::thread_rng().gen_range(0, 3);
    //     match direction {
    //         0 => self.move_by(0, 1, map),
    //         2 => self.move_by(0, -1, map),
    //         3 => self.move_by(1, 0, map),
    //         4 => self.move_by(-1, 0, map),
    //         _ => {}
    //     };
    // }

    pub fn get_pos(&self) -> (i32, i32) {
        return (self.x, self.y);
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn move_by(idx: usize, dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let (x, y) = objects[idx].get_pos();

    let new_x = x + dx;
    let new_y = y + dy;

    if !is_blocked(new_x, new_y, map, objects) {
        objects[idx].set_pos(new_x, new_y);
    }
}

pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    if map.map[x as usize][y as usize].blocked {
        return true;
    }

    objects
        .iter()
        .any(|object| object.blocks && object.get_pos() == (x, y))
}
