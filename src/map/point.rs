pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn origin() -> Point {
        Point{x: 0, y : 0}
    }

    pub fn new(x : i32, y : i32) -> Point { 
        Point { x : x, y : y}
    }
}