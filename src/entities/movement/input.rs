use entities::object::Object;
use map::map::Map;

pub trait Input {
    fn update(&mut self, object : &mut Object, map : &Map);
}