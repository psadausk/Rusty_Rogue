extern crate rand;

use entities::object::Ai;
use entities::object::DeathCallback;
use entities::object::Fighter;
use entities::object::Object;

use crate::entities::light;
use crate::entities::light::LightSource;

use self::rand::Rng;
use map::rect::Rect;
use map::shadow_line::Shadow;
use map::shadow_line::ShadowLine;
use map::tile::Tile;
use std::cmp;
use tcod::colors;
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const DEFAULT_SHADE_FACTOR: f32 = 0.8;
const MAX_ROOM_MONSTERS: i32 = 3;

pub struct Map {
    pub map: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Self {
        Map {
            map: vec![
                vec![Tile::wall(0, 0, DEFAULT_SHADE_FACTOR); height as usize];
                width as usize
            ],
            width: width,
            height: height,
        }
    }

    pub fn is_in_fov(&self, object: &Object) -> bool {
        let (x, y) = object.get_pos();
        return self.map[x as usize][y as usize].visible;
    }

    pub fn make_rand_map(&mut self) -> Vec<Object> {
        let mut rooms = vec![];
        let mut objects = vec![];
        for _ in 0..MAX_ROOMS {
            let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = rand::thread_rng().gen_range(0, self.width - w);
            let y = rand::thread_rng().gen_range(0, self.height - h);

            let new_room = Rect::new(x, y, w, h);
            let failed = rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));

            if !failed {
                self.create_room(new_room);
                let (new_x, new_y) = new_room.center();

                if rooms.is_empty() {
                    //start = (new_x, new_y);
                    let mut player = Object::new(new_x, new_y, '@', "player", colors::WHITE, true);
                    player.alive = true;
                    player.fighter = Some(Fighter {
                        max_hp: 30,
                        hp: 30,
                        defense: 2,
                        power: 5,
                        on_death: DeathCallback::Player,
                    });
                    player.light = Some(LightSource::new(3.0, 8.0));
                    objects.push(player);
                } else {
                    // all rooms after the first:
                    // connect it to the previous room with a tunnel

                    // center coordinates of the previous room
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                    // draw a coin (random bool value -- either true or false)
                    if rand::random() {
                        // first move horizontally, then vertically
                        self.create_h_tunnel(prev_x, new_x, prev_y);
                        self.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        // first move vertically, then horizontally
                        self.create_v_tunnel(prev_y, new_y, prev_x);
                        self.create_h_tunnel(prev_x, new_x, new_y);
                    }
                    //Now add monsters to the room
                    self.place_object(new_room, &mut objects)
                }

                // finally, append the new room to the list
                rooms.push(new_room);
            }
        }
        objects
    }

    fn create_room(&mut self, room: Rect) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                self.map[x as usize][y as usize] = Tile::floor(x, y, DEFAULT_SHADE_FACTOR);
            }
        }
    }

    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..cmp::max(x1, x2) + 1 {
            self.map[x as usize][y as usize] = Tile::floor(x, y, DEFAULT_SHADE_FACTOR);
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..cmp::max(y1, y2) + 1 {
            self.map[x as usize][y as usize] = Tile::floor(x, y, DEFAULT_SHADE_FACTOR);
        }
    }

    //Visibility
    pub fn refresh_lights(&mut self, x: i32, y: i32, light_source: &LightSource) {
        println!();
        println!();
        println!("new refresh");
        for octant in 0..8 {
            self.refresh_octant(x, y, octant, light_source);
        }
    }

    fn refresh_octant(&mut self, x: i32, y: i32, octant: i32, light_source: &LightSource) {
        let mut line = ShadowLine::new();
        let mut full_shadow = false;
        let mut row = 1;
        //TODO: set limit to how far we can see
        //let radius = 20;

        loop {
            let (x_transform, y_transform) = self.transform_octant(row, 0, octant);
            let (pos_x, pos_y) = (x + x_transform, y + y_transform);

            if pos_x < 0 || pos_x >= self.width {
                break;
            }

            if pos_y < 0 || pos_y >= self.height {
                break;
            }

            for col in 0..row + 1 {
                //if out of bounds, break
                let (x_transform, y_transform) = self.transform_octant(row, col, octant);
                let (pos_x, pos_y) = (x + x_transform, y + y_transform);

                if pos_x < 0 || pos_x >= self.width {
                    break;
                }

                if pos_y < 0 || pos_y >= self.height {
                    break;
                }

                if full_shadow {
                    self.map[pos_x as usize][pos_y as usize].visible = false;
                    self.map[pos_x as usize][pos_y as usize].shade_factor = DEFAULT_SHADE_FACTOR
                } else {
                    let projection = self.project_tile(row as f32, col as f32);
                    let visible = !line.is_in_shadow(&projection);
                    if visible && self.map[pos_x as usize][pos_y as usize].block_sight {
                        line.add(projection);
                        full_shadow = line.is_full_shadow();
                    }
                    let dist = ((i32::pow(row, 2) + i32::pow(col, 2)) as f64).sqrt() as f32;
                    if (visible && dist < light_source.max_dist) {
                        self.map[pos_x as usize][pos_y as usize].shade_factor =
                            self.get_shade_factor(row, col, light_source);
                        self.map[pos_x as usize][pos_y as usize].visible = true;
                    } else {
                        self.map[pos_x as usize][pos_y as usize].shade_factor =
                            DEFAULT_SHADE_FACTOR;
                        self.map[pos_x as usize][pos_y as usize].visible = false;
                    }
                }
            }
            row = row + 1;
        }
        //for row in 1..
    }

    fn get_shade_factor(&self, x: i32, y: i32, light_source: &LightSource) -> f32 {
        //println!("x: {0}, y: {1}", x, y);
        let distance = ((x * x + y * y) as f64).sqrt() as f32;

        //println!("distance: {0}", distance);
        let intensity = light_source.calc_shade_percent(distance);

        let ret = f32::min(DEFAULT_SHADE_FACTOR, (intensity) as f32);
        println!("intensity: {0}", intensity);
        return ret;
    }

    fn project_tile(&self, row: f32, col: f32) -> Shadow {
        let top_left = col / (row + 2.0);
        let bottom_right = (col + 1.0) / (row + 1.0);
        Shadow::new(top_left, bottom_right)
    }

    // octants represent the 8 spaces around the player
    fn transform_octant(&self, row: i32, col: i32, octant: i32) -> (i32, i32) {
        match octant {
            0 => return (col, -row),
            1 => return (row, -col),
            2 => return (row, col),
            3 => return (col, row),
            4 => return (-col, row),
            5 => return (-row, col),
            6 => return (-row, -col),
            _ => return (-col, -row),
        }
    }

    //Monsters

    fn place_object(&mut self, room: Rect, objects: &mut Vec<Object>) {
        let num_monsters = rand::thread_rng().gen_range(0, MAX_ROOM_MONSTERS + 1);

        for _ in 0..num_monsters {
            let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
            let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

            let mut monster = if rand::random::<f32>() < 0.8 {
                let mut orc = Object::new(x, y, 'o', "Orc", colors::DESATURATED_GREEN, true);
                orc.fighter = Some(Fighter {
                    max_hp: 10,
                    hp: 10,
                    defense: 0,
                    power: 3,
                    on_death: DeathCallback::Monster,
                });
                orc.ai = Some(Ai::Basic);
                orc
            } else {
                let mut troll = Object::new(x, y, 'T', "Troll", colors::DARKER_GREEN, true);
                troll.fighter = Some(Fighter {
                    max_hp: 16,
                    hp: 16,
                    defense: 1,
                    power: 4,
                    on_death: DeathCallback::Monster,
                });
                troll.ai = Some(Ai::Basic);
                troll
            };
            monster.alive = true;
            objects.push(monster);
        }
    }
}
