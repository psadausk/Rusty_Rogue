extern crate rand;
extern crate tcod;

use entities::object::Object;
use map::map::Map;
use tcod::console::*;
use tcod::console::{Offscreen, Root};

use crate::entities::object::move_by;

mod entities;
mod map;

//use KeyCode::{Up, Down, Left, Right, Escape};
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;

const PLAYER_IDX: usize = 0;

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
    let mut objects = map.make_rand_map();

    let mut prev_pos = (-1, -1);

    tcod::system::set_fps(LIMIT_FPS);

    while !root.window_closed() {
        let fov_recompute = prev_pos != objects[0].get_pos();
        render_all(&mut root, &mut con, &mut objects, &mut map, fov_recompute);

        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con)
        }

        // handle keys and exit game if needed
        let player = &mut objects[0];
        prev_pos = player.get_pos();
        let player_action = handle_keys(&mut root, &map, &mut objects);
        if player_action == PlayerAction::Exit {
            break;
        }

        // let monsters take their turn
        if objects[PLAYER_IDX].alive && player_action != PlayerAction::DidntTakeTurn {
            for object in &objects {
                // only if object is not player
                if (object as *const _) != (&objects[PLAYER_IDX] as *const _) {
                    println!("The {} growls!", object.name);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

fn handle_keys(root: &mut Root, map: &Map, objects: &mut Vec<Object>) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use PlayerAction::*;

    let key = root.wait_for_keypress(true);
    let player_alive = objects[0].alive;
    match (key, key.text(), player_alive) {
        (
            Key {
                code: Enter,
                alt: true,
                ..
            },
            _,
            _,
        ) => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
            return DidntTakeTurn;
        }
        (Key { code: Escape, .. }, _, true) => return Exit, // exit game

        // movement keys
        //Key { code: Spacebar, .. } => {}
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, &map, objects);
            return TookTurn;
        }
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, &map, objects);
            return TookTurn;
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, &map, objects);
            return TookTurn;
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, &map, objects);
            return TookTurn;
        }

        _ => return DidntTakeTurn,
    }
}

fn player_move_or_attack(dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    // the coordinates the player is moving to/attacking
    let (x, y) = objects[PLAYER_IDX].get_pos();

    let new_x = x + dx;
    let new_y = y + dy;

    // try to find an attackable object there
    let target_id = objects
        .iter()
        .position(|object| object.get_pos() == (new_x, new_y));

    // attack if target found, move otherwise
    match target_id {
        Some(target_id) => {
            println!(
                "The {} laughs at your puny efforts to attack him!",
                objects[target_id].name
            );
        }
        None => {
            move_by(PLAYER_IDX, dx, dy, &map, objects);
        }
    }
}

fn render_all(
    root: &mut Root,
    con: &mut Offscreen,
    objects: &mut [Object],
    map: &mut Map,
    fov_recompute: bool,
) {
    if fov_recompute {
        let (x, y) = objects[0].get_pos();
        map.refresh_visibility(x, y);
    }

    for object in objects {
        let (x, y) = object.get_pos();
        let tile = map.map[x as usize][y as usize];
        if tile.visible {
            object.draw(con);
        }
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
