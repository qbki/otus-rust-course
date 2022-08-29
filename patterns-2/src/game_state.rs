use crate::common::*;
use crate::point::*;

pub struct GameState {
    pub player: Player,
    pub foot_print: Vec<Point>,
}

pub struct GameStateBuilder {
    player: Player,
    foot_print: Vec<Point>,
}

impl GameStateBuilder {
    pub fn new() -> Self {
        GameStateBuilder {
            player: Player {
                pos: Point::new(0, 0),
                sign: '?',
            },
            foot_print: vec![],
        }
    }

    pub fn player_position(&mut self, x: i32, y: i32) -> &mut Self {
        self.player.pos.x = x;
        self.player.pos.y = y;
        self
    }

    pub fn player_sign(&mut self, sign: char) -> &mut Self {
        self.player.sign = sign;
        self
    }

    pub fn foot_print(&mut self, foot_print: Vec<Point>) -> &mut Self {
        self.foot_print.extend(foot_print);
        self
    }

    pub fn build(&self) -> GameState {
        GameState {
            player: self.player,
            foot_print: self.foot_print.clone(),
        }
    }
}
