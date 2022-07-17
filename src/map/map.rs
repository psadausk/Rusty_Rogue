extern crate rand;

use map::rect::Rect;
use map::shadow_line::Shadow;
use map::shadow_line::ShadowLine;
use map::tile::Tile;
use rand::Rng;
use std::cmp;
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;
const DEFAULT_SHADE_FACTOR: f32 = 0.7;

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

    pub fn make_fixed_map(&mut self) -> (i32, i32) {
        let room = Rect::new(20, 20, 15, 15);
        self.create_room(room);

        self.map[23][24] = Tile::wall(23, 24, DEFAULT_SHADE_FACTOR);
        self.map[25][23] = Tile::wall(25, 23, DEFAULT_SHADE_FACTOR);
        self.map[25][24] = Tile::wall(25, 24, DEFAULT_SHADE_FACTOR);
        self.map[25][25] = Tile::wall(25, 25, DEFAULT_SHADE_FACTOR);

        (22, 27)
    }

    pub fn make_rand_map(&mut self) -> (i32, i32) {
        // let room1 = Rect::new(20, 15, 10, 15);
        // let room2 = Rect::new(50, 15, 10, 15);
        // self.create_room(room1);
        // self.create_room(room2);
        // self.create_h_tunnel(25, 55, 23);

        let mut rooms = vec![];
        let mut start = (0, 0);
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
                    start = (new_x, new_y);
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
                }
                // finally, append the new room to the list
                rooms.push(new_room);
            }
        }
        start
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

    pub fn refresh_visibility(&mut self, x: i32, y: i32) {
        for octant in 0..8 {
            self.refresh_octant(x, y, octant);
        }
    }

    fn refresh_octant(&mut self, x: i32, y: i32, octant: i32) {
        let mut line = ShadowLine::new();
        let mut full_shadow = false;
        let mut row = 1;
        let radius = 20;

        while true {
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
                    self.map[pos_x as usize][pos_y as usize].shade_factor = 0.7
                } else {
                    let projection = self.project_tile(row as f32, col as f32);
                    let visible = !line.is_in_shadow(&projection);
                    self.map[pos_x as usize][pos_y as usize].visible = visible;
                    if visible && self.map[pos_x as usize][pos_y as usize].block_sight {
                        line.add(projection);
                        full_shadow = line.is_full_shadow();
                    }
                    if visible {
                        self.map[pos_x as usize][pos_y as usize].shade_factor =
                            self.get_shade_factor(row, col);
                    } else {
                        self.map[pos_x as usize][pos_y as usize].shade_factor = 0.7
                    }
                }
            }
            row = row + 1;
        }
        //for row in 1..
    }

    fn get_shade_factor(&self, x: i32, y: i32) -> f32 {
        let square = (x * x + y * y) as f32;
        //println!("square: {0}", square);
        let intesity = 1.0 - (square / (20.0 * 20.0));
        //println!("intensity: {0}", intesity);
        let ret = f32::min(0.7, (1.0 - intesity) as f32);
        //let ret = f32::min(0.7, (distance / 20.0) as f32);
        return ret;
    }

    fn project_tile(&self, row: f32, col: f32) -> Shadow {
        let topLeft = col / (row + 2.0);
        let bottomRight = (col + 1.0) / (row + 1.0);
        Shadow::new(topLeft, bottomRight)
    }

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
}
