extern crate rand;
extern crate tcod;

use std::{cmp, env};

use entities::light;
use entities::object::Object;

use map::map::Map;
use map::movement_helper::{move_by, move_towards};
use tcod::console::{Offscreen, Root};
use tcod::input::{self, Event, Key, Mouse};
use tcod::{colors, console::*, Color};
use ui::messages::Messages;

mod entities;
mod map;
mod ui;

//use KeyCode::{Up, Down, Left, Right, Escape};
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

const PLAYER_IDX: usize = 0;

// sizes and coordinates relevant for the GUI
const BAR_WIDTH: i32 = 20;
const PANEL_HEIGHT: i32 = 7;
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT;

//Message panel
const MSG_X: i32 = BAR_WIDTH + 2;
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH - 2;
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1;

struct Game {
    map: Map,
    messages: Messages,
}

struct Tcod {
    root: Root,
    con: Offscreen,
    panel: Offscreen,
    key: Key,
    mouse: Mouse,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    print!("inited");
    let mut root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Rogue")
        .init();

    let mut tcod = Tcod {
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        panel: Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT),
        key: Default::default(),
        mouse: Default::default(),
    };

    let mut game = Game {
        map: Map::new(MAP_WIDTH, MAP_HEIGHT),
        messages: Messages::new(),
    };
    let mut objects = game.map.make_rand_map();

    let mut prev_pos = (-1, -1);

    tcod::system::set_fps(LIMIT_FPS);

    game.messages.add("Welcome stranger!", colors::RED);

    while !tcod.root.window_closed() {
        tcod.con.clear();
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => tcod.mouse = m,
            Some((_, Event::Key(k))) => tcod.key = k,
            _ => tcod.key = Default::default(),
        };

        let fov_recompute = prev_pos != objects[0].get_pos();
        //println!("rendering");
        render_all(&mut tcod, &mut objects, &mut game, fov_recompute);

        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut tcod.con)
        }

        // handle keys and exit game if needed
        //println!("handling player movement");
        let player = &mut objects[0];
        prev_pos = player.get_pos();
        let player_action = handle_keys(&mut tcod, &mut game, &mut objects);
        if player_action == PlayerAction::Exit {
            break;
        }

        // let monsters take their turn
        //println!("monster turns");
        if objects[PLAYER_IDX].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                // only if object is not player
                if (objects[id].ai.is_some()) {
                    ai_turn(id, &mut game, &mut objects);
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

fn handle_keys(tcod: &mut Tcod, game: &mut Game, objects: &mut Vec<Object>) -> PlayerAction {
    use tcod::input::KeyCode::*;
    use PlayerAction::*;

    let player_alive = objects[0].alive;
    match (tcod.key, tcod.key.text(), player_alive) {
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
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            return DidntTakeTurn;
        }
        (Key { code: Escape, .. }, _, true) => return Exit, // exit game

        // movement keys
        //Key { code: Spacebar, .. } => {}
        (Key { code: Up, .. }, _, true) => {
            player_move_or_attack(0, -1, game, objects);
            return TookTurn;
        }
        (Key { code: Down, .. }, _, true) => {
            player_move_or_attack(0, 1, game, objects);
            return TookTurn;
        }
        (Key { code: Left, .. }, _, true) => {
            player_move_or_attack(-1, 0, game, objects);
            return TookTurn;
        }
        (Key { code: Right, .. }, _, true) => {
            player_move_or_attack(1, 0, game, objects);
            return TookTurn;
        }

        _ => return DidntTakeTurn,
    }
}

fn player_move_or_attack(dx: i32, dy: i32, game: &mut Game, objects: &mut [Object]) {
    // the coordinates the player is moving to/attacking
    let (x, y) = objects[PLAYER_IDX].get_pos();

    let new_x = x + dx;
    let new_y = y + dy;

    // try to find an attackable object there
    let target_id = objects
        .iter()
        .position(|object| object.fighter.is_some() && object.get_pos() == (new_x, new_y));

    // attack if target found, move otherwise
    match target_id {
        Some(target_id) => {
            let (player, target) = mut_two(PLAYER_IDX, target_id, objects);
            player.attack(target, game);
        }
        None => {
            move_by(PLAYER_IDX, dx, dy, &game.map, objects);
        }
    }
}

fn render_all(tcod: &mut Tcod, objects: &[Object], game: &mut Game, fov_recompute: bool) {
    if fov_recompute {
        // let (x, y) = objects[0].get_pos();
        // game.map.refresh_visibility(x, y);

        for obj in objects {
            // let (x, y) = objects[idx].get_pos();
            // match(objects[idx])
            if let Some(light) = &obj.light {
                let (x, y) = obj.get_pos();
                game.map.refresh_lights(x, y, light)
            }
        }
    }

    let mut to_draw: Vec<_> = objects.iter().filter(|o| game.map.is_in_fov(o)).collect();
    to_draw.sort_by(|o1, o2| o1.blocks.cmp(&o2.blocks));
    for object in to_draw {
        let (x, y) = object.get_pos();
        let tile = game.map.map[x as usize][y as usize];
        if tile.visible {
            //object.draw(&mut tcod.con);
            let color = shade_color(
                object.color,
                game.map.map[x as usize][y as usize].shade_factor,
            );
            tcod.con.set_default_foreground(color);
            let (x, y) = object.get_pos();
            tcod.con.put_char(x, y, object.char, BackgroundFlag::None);
        }
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let visible = game.map.map[x as usize][y as usize].visible;

            let color = shade_color(
                game.map.map[x as usize][y as usize].color,
                game.map.map[x as usize][y as usize].shade_factor,
            );

            //println!("color: {0}", color);
            let explored = &mut game.map.map[x as usize][y as usize].explored;
            if visible {
                *explored = true;
            }
            if *explored {
                tcod.con
                    .set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }
    // show the player's stats
    tcod.panel.set_default_background(colors::BLACK);
    tcod.panel.clear();
    let hp = objects[PLAYER_IDX].fighter.map_or(0, |f| f.hp);
    let max_hp = objects[PLAYER_IDX].fighter.map_or(0, |f| f.max_hp);
    render_bar(
        &mut tcod.panel,
        1,
        1,
        BAR_WIDTH,
        "HP",
        hp,
        max_hp,
        colors::LIGHT_RED,
        colors::DARKER_RED,
    );

    // display names of objects under the mouse
    tcod.panel.set_default_foreground(colors::LIGHT_GREY);
    tcod.panel.print_ex(
        1,
        0,
        BackgroundFlag::None,
        TextAlignment::Left,
        get_names_under_mouse(tcod.mouse, objects, &game.map),
    );

    // print the game messages, one line at a time
    let mut y = MSG_HEIGHT as i32;
    for &(ref msg, color) in game.messages.iter().rev() {
        let msg_height = tcod.panel.get_height_rect(MSG_X, y, MSG_WIDTH, 0, msg);
        y -= msg_height;
        if y < 0 {
            break;
        }
        tcod.panel.set_default_foreground(color);
        tcod.panel.print_rect(MSG_X, y, MSG_WIDTH, 0, msg);
    }

    blit(
        &mut tcod.panel,
        (0, 0),
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut tcod.root,
        (0, PANEL_Y),
        1.0,
        1.0,
    );
    // blit the contents of "con" to the root console and present it
    blit(
        &mut tcod.con,
        (0, 0),
        (MAP_WIDTH, MAP_HEIGHT),
        &mut tcod.root,
        (0, 0),
        1.0,
        1.0,
    );
    tcod.root.flush();
}

fn shade_color(original: Color, shade_factor: f32) -> Color {
    let mut color: Color = colors::WHITE;
    color.r = ((original.r as f32) * (1.0 - shade_factor)) as u8;
    color.g = ((original.g as f32) * (1.0 - shade_factor)) as u8;
    color.b = ((original.b as f32) * (1.0 - shade_factor)) as u8;
    return color;
}

fn ai_turn(monster_id: usize, game: &mut Game, objects: &mut [Object]) {
    let (monster_x, monster_y) = objects[monster_id].get_pos();
    let monster_tile = game.map.map[monster_x as usize][monster_y as usize];
    if monster_tile.visible {
        if objects[monster_id].distance_to(&objects[PLAYER_IDX]) >= 2.0 {
            let (player_x, player_y) = objects[PLAYER_IDX].get_pos();
            move_towards(monster_id, player_x, player_y, &game.map, objects);
        } else if objects[PLAYER_IDX].fighter.map_or(false, |f| f.hp > 0) {
            let (monster, player) = mut_two(monster_id, PLAYER_IDX, objects);
            monster.attack(player, game);
        }
    }
}

/// Mutably borrow two *separate* elements from the given slice.
/// Panics when the indexes are equal or out of bounds.
fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert!(first_index != second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let (first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

fn render_bar(
    panel: &mut Offscreen,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,
) {
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    panel.set_default_background((back_color));
    panel.rect(x, y, total_width, 1, false, BackgroundFlag::Screen);

    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, false, BackgroundFlag::Screen);
    }

    panel.set_default_foreground(colors::WHITE);
    panel.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Center,
        &format!("{}: {}/{}", name, value, maximum),
    );
}

fn get_names_under_mouse(mouse: Mouse, objects: &[Object], map: &Map) -> String {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);

    // create a list with the names of all objects at the mouse's coordinates and in FOV
    let names = objects
        .iter()
        .filter(|obj| obj.get_pos() == (x, y) && map.is_in_fov(obj))
        .map(|obj| obj.name.clone())
        .collect::<Vec<_>>();

    names.join(", ") // join the names, separated by commas
}
