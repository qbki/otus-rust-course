use crate::point::*;

pub struct Player {
    pub pos: Point,
    pub sign: char,
}

pub struct GameState {
    pub player: Player,
    pub foot_print: Vec<Point>,
}
