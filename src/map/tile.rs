use tcod::colors::{self, Color};

const COLOR_DARK_WALL: Color = Color { r: 51, g: 21, b: 0 };
const COLOR_DARK_FLOOR: Color = Color {
    r: 80,
    g: 64,
    b: 20,
};

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
    pub visible: bool, //TODO: Eventually this should be a continous thing, not a bool
    pub x: i32,
    pub y: i32,
    pub shade_factor: f32,
    pub color: Color,
}

impl Tile {
    pub fn floor(x: i32, y: i32, shade_factor: f32) -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            explored: false,
            visible: false,
            x: x,
            y: y,
            shade_factor: shade_factor,
            color: COLOR_DARK_FLOOR,
        }
    }

    pub fn wall(x: i32, y: i32, shade_factor: f32) -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            explored: false,
            visible: false,
            x: x,
            y: y,
            shade_factor: shade_factor,
            color: COLOR_DARK_WALL,
        }
    }
}
