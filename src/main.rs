#![allow(dead_code)]
extern crate tcod;

use tcod::console::*;
use tcod::colors::{self, Color};
use tcod::input::*;
use tcod::console::{self, Root, Offscreen};
use object::Object;
use tile::Tile;
use rect::Rect;

use std::cmp;

//mod cmp;
mod tile;
mod object;
mod rect;

//use KeyCode::{Up, Down, Left, Right, Escape};
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL : Color = Color { r: 51, g: 21, b: 0};
const COLOR_DARK_FLOOR : Color = Color { r: 80, g: 64, b: 20};

type Map = Vec<Vec<Tile>>;


pub enum Direction {
    N,
    S,
    W,
    E,
}


fn main() {
    let mut root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Rogue")
        .init();
    
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let player = Object::new(25, 23, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut objects = [player, npc];
    let map = make_map();

    //con.set_char_foreground


    tcod::system::set_fps(LIMIT_FPS);


    while !root.window_closed() {
    render_all(&mut root, &mut con, &objects, &map);



        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con)
        }

        // handle keys and exit game if needed
        
        let exit = handle_keys(&mut root, &mut objects[0], &map);
        if exit {
            break
        }
    }

}

fn handle_keys(root: &mut Root, player: &mut Object, map :&Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,  // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1, &map),
        Key { code: Down, .. } => player.move_by(0, 1, &map),
        Key { code: Left, .. } => player.move_by(-1, 0, &map),
        Key { code: Right, .. } => player.move_by(1, 0, &map),

        _ => {},
    }

    false
}

fn render_all(root: &mut Root, con : &mut Offscreen, objects: &[Object], map: &Map){
    for object in objects {
        object.draw(con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][ y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_FLOOR, BackgroundFlag::Set);
            }
        }
    }
            // blit the contents of "con" to the root console and present it
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
    root.flush();
}


fn make_map() -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    
    // map[30][22] = Tile::wall();
    // map[50][22] = Tile::wall();

    let room1 = Rect::new(20,15,10,15);
    let room2 = Rect::new(50,15,10,15);
    create_room(room1, &mut map);
    create_room(room2, &mut map);
    create_h_tunnel(25, 55, 23, &mut map);
    map
}

    fn create_room(room: Rect, map: &mut Map){
        for x in (room.x1 + 1) .. room.x2 {
            for y in (room.y1 + 1) .. room.y2 {
                map[x as usize][y as usize] = Tile::empty();
            }
        }
    }

    fn create_h_tunnel(x1: i32, x2: i32, y: i32, map : &mut Map){
        for x in cmp::min(x1, x2) .. cmp::max(x1, x2) + 1{
            map[x as usize][y as usize] = Tile::empty();
        }
    }

        fn create_v_tunnel(y1: i32, y2: i32, x: i32, map : &mut Map){
        for y in cmp::min(y1, y2) .. cmp::max(y1, y2) + 1{
            map[x as usize][y as usize] = Tile::empty();
        }
    }
