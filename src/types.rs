pub struct Dimensions {
    pub window_width: i32,
    pub window_height: i32,
    pub half_width: i32,
    pub half_height: i32,
    pub lane_width: i32,
    pub speed: Speed,
}

pub struct Speed {
    pub fast: i32,
    pub default: i32,
    pub slow: i32,
}

// These directions are all from our point of view as we look at the screen. They describe a car's initial direction and its direction after it's turned, both from our perspective.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Airt {
    Up,
    Down,
    Left,
    Right,
}
