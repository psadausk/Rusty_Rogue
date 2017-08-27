
extern crate tcod;

use tcod::colors::{self, Color};
use tcod::console::*;
use tile::Tile;

pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,

}

type Map = Vec<Vec<Tile>>;

impl Object {
    pub fn new(x: i32, y:i32, char:char, color:Color) -> Self {        
        Object { x:x, y:y, char:char, color:color}
    }

    pub fn move_by(&mut self, dx:i32, dy:i32, map : &Map){
        if !map[(self.x + dx ) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        }
    }

    pub fn draw(&self, con: &mut Console){
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn clear(&self, con : &mut Console){
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }

}