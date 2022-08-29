use crate::game_state::GameState;
use termion::terminal_size;

pub trait Command {
    fn execute(&self, state: &mut GameState);
    fn rollback(&self, state: &mut GameState);
}

pub struct MoveUpCommand();
pub struct MoveDownCommand();
pub struct MoveLeftCommand();
pub struct MoveRightCommand();
pub struct NoopCommand();

impl Command for MoveUpCommand {
    fn execute(&self, state: &mut GameState) {
        move_player_forward(0, -1, state);
    }

    fn rollback(&self, state: &mut GameState) {
        move_player_backward(state);
    }
}

impl Command for MoveDownCommand {
    fn execute(&self, state: &mut GameState) {
        move_player_forward(0, 1, state);
    }

    fn rollback(&self, state: &mut GameState) {
        move_player_backward(state);
    }
}

impl Command for MoveLeftCommand {
    fn execute(&self, state: &mut GameState) {
        move_player_forward(-1, 0, state);
    }

    fn rollback(&self, state: &mut GameState) {
        move_player_backward(state);
    }
}

impl Command for MoveRightCommand {
    fn execute(&self, state: &mut GameState) {
        move_player_forward(1, 0, state);
    }

    fn rollback(&self, state: &mut GameState) {
        move_player_backward(state);
    }
}

impl Command for NoopCommand {
    fn execute(&self, _: &mut GameState) {}
    fn rollback(&self, _: &mut GameState) {}
}

fn move_player_forward(x: i32, y: i32, state: &mut GameState) {
    let mut pos = state.player.pos.clone();
    let terminal_size = terminal_size().unwrap();
    pos.x += x;
    pos.y += y;
    let is_moved =
        pos.x > 0
        && pos.y > 0
        && pos.x < terminal_size.0 as i32
        && pos.y < terminal_size.1 as i32;
    if is_moved {
        state.player.pos = pos;
        state.foot_print.push(pos);
    }
}

fn move_player_backward(state: &mut GameState) {
    state.foot_print.pop();
    if let Some(pos) = state.foot_print.last() {
        state.player.pos = *pos;
    }
}
