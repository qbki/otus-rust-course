extern crate termion;

mod commands;
mod common;
mod game_state;
mod point;

use crate::commands::*;
use crate::game_state::*;
use crate::point::*;
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const FOOT_PRINT_SIGN: char = '.';

fn draw_frame(state: &GameState, stdout: &mut Stdout) {
    let terminal_size = termion::terminal_size().unwrap();
    println!("{}", termion::clear::All);
    for item in &state.foot_print {
        println!(
            "{}{}",
            termion::cursor::Goto(item.x as u16, item.y as u16),
            FOOT_PRINT_SIGN,
        );
    }
    println!(
        "{}Use ←↑↓→ keys to move around",
        termion::cursor::Goto(1, 2)
    );
    println!(
        "{}Backspace returns you to the previous position",
        termion::cursor::Goto(1, 3)
    );
    println!("{}Esc - exit the game", termion::cursor::Goto(1, 4));
    println!(
        "{}{}",
        termion::cursor::Goto(state.player.pos.x as u16, state.player.pos.y as u16),
        state.player.sign
    );
    println!(
        "{}",
        termion::cursor::Goto(terminal_size.0 - 2, terminal_size.1 - 2)
    );
    stdout.flush().unwrap();
}

fn main() {
    let stdin = stdin();
    let mut commands: Vec<Box<dyn Command>> = vec![Box::new(NoopCommand())];
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut state = GameStateBuilder::new()
        .player_position(20, 10)
        .player_sign('@')
        .foot_print(vec![Point::new(20, 10)])
        .build();

    draw_frame(&state, &mut stdout);
    for event_result in stdin.events() {
        let event = event_result.unwrap();
        let command: Box<dyn Command> = match event {
            Event::Key(Key::Up) => Box::new(MoveUpCommand()),
            Event::Key(Key::Down) => Box::new(MoveDownCommand()),
            Event::Key(Key::Left) => Box::new(MoveLeftCommand()),
            Event::Key(Key::Right) => Box::new(MoveRightCommand()),
            Event::Key(Key::Esc) => {
                println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                return;
            }
            Event::Key(Key::Backspace) => {
                if let Some(rollback_command) = commands.pop() {
                    rollback_command.rollback(&mut state);
                }
                draw_frame(&state, &mut stdout);
                continue;
            }
            _ => Box::new(NoopCommand()),
        };

        command.execute(&mut state);
        draw_frame(&state, &mut stdout);
        commands.push(command);
    }
}
