use entities::object::Object;
use map::map::Map;

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

pub fn move_towards(id: usize, target_x: i32, target_y: i32, map: &Map, objects: &mut [Object]) {
    // vector from this object to the target, and distance
    let (x, y) = objects[id].get_pos();
    let dx = target_x - x;
    let dy = target_y - y;

    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    // normalize it to length 1 (preserving direction), then round it and
    // convert to integer so the movement is restricted to the map grid
    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, map, objects);
}
