#![allow(dead_code)]
extern crate rand;
extern crate tcod;
//extern crate std;

//use entities::movement::player_movement::PlayerMovement;
use entities::object::Object;
use map::map::Map;
use std::boxed::Box;
use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::console::{self, Offscreen, Root};
use tcod::map::{FovAlgorithm, Map as FovMap};

mod entities;
mod map;

//use KeyCode::{Up, Down, Left, Right, Escape};
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const COLOR_DARK_WALL: Color = Color { r: 51, g: 21, b: 0 };
const COLOR_DARK_FLOOR: Color = Color {
    r: 80,
    g: 64,
    b: 20,
};
const COLOR_LIGHT_WALL: Color = Color { r: 84, g: 35, b: 0 };
const COLOR_LIGHT_FLOOR: Color = Color {
    r: 200,
    g: 180,
    b: 50,
};

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

fn main() {
    print!("inited");
    let mut root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Rogue")
        .init();

    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);
    let (player_pos_x, player_pos_y) = map.make_rand_map();
    let player = Object::new(player_pos_x, player_pos_y, '@', colors::WHITE);

    print!("Made the map");
    // //let npc = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut objects = [player];

    // let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    // for y in 0..MAP_HEIGHT {
    //     for x in 0..MAP_WIDTH {
    //         fov_map.set(x, y, !map.map[x as usize][y as usize].block_sight, !map.map[x as usize][y as usize].blocked);
    //     }
    // }

    let mut prev_pos = (-1, -1);

    tcod::system::set_fps(LIMIT_FPS);

    while !root.window_closed() {
        let fov_recompute = prev_pos != (objects[0].x, objects[0].y);
        render_all(&mut root, &mut con, &mut objects, &mut map, fov_recompute);

        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con)
        }

        // handle keys and exit game if needed
        let player = &mut objects[0];
        prev_pos = (player.x, player.y);
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true, // exit game

        // movement keys
        Key { code: Spacebar, .. } => {}
        Key { code: Up, .. } => player.move_by(0, -1, &map),
        Key { code: Down, .. } => player.move_by(0, 1, &map),
        Key { code: Left, .. } => player.move_by(-1, 0, &map),
        Key { code: Right, .. } => player.move_by(1, 0, &map),

        _ => {}
    }

    false
}

fn render_all(
    root: &mut Root,
    con: &mut Offscreen,
    objects: &mut [Object],
    map: &mut Map,
    fov_recompute: bool,
) {
    if fov_recompute {
        let player = &mut objects[0];
        map.refresh_visibility(player.x, player.y);
    }

    for object in objects {
        //if fov_map.is_in_fov(object.x, object.y) {
        object.draw(con);
        //}
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = map.map[x as usize][y as usize].visible;

            let mut color = map.map[x as usize][y as usize].color;

            color.r =
                ((color.r as f32) * (1.0 - map.map[x as usize][y as usize].shade_factor)) as u8;
            color.g =
                ((color.g as f32) * (1.0 - map.map[x as usize][y as usize].shade_factor)) as u8;
            color.b =
                ((color.b as f32) * (1.0 - map.map[x as usize][y as usize].shade_factor)) as u8;
            let explored = &mut map.map[x as usize][y as usize].explored;
            if visible {
                *explored = true;
            }
            if *explored {
                con.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }
    // blit the contents of "con" to the root console and present it
    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
    root.flush();
}
