use super::room_management::room_management;
use super::utils::{get_input, get_name, wrong_command};
use crate::common::{Report, RequestType, SmartHomeErrorEnum};
use crate::smart_home::SmartHome;
use crate::smart_room::SmartRoom;
use std::io;

#[derive(PartialEq)]
enum TuiState {
    Exit,
    Greeting,
    WrongCommand,
    AddRoom,
    RoomManagement,
    Report,
}

fn greeting(writer: &mut dyn io::Write) -> io::Result<()> {
    writeln!(writer, "Welcome to Smart Home Command Line Interface")
}

fn add_room(writer: &mut dyn io::Write, home: &mut SmartHome) -> io::Result<()> {
    writeln!(writer, "Please enter a room name:")?;

    let name = get_name(writer)?;
    home.add_room(&name);

    Result::Ok(())
}

fn handle_rooms(writer: &mut dyn io::Write, home: &mut SmartHome) -> io::Result<()> {
    writeln!(writer, "Please enter a room name:")?;

    let name = get_name(writer)?;
    let room_result: Result<&mut SmartRoom, SmartHomeErrorEnum> =
        home.get(&RequestType::Room(&name)).into();

    match room_result {
        Result::Ok(room) => room_management(writer, room),
        Result::Err(error) => writeln!(writer, "{}", error),
    }
}

pub fn home_management(writer: &mut dyn io::Write, home: &mut SmartHome) -> io::Result<()> {
    let mut state = TuiState::Greeting;

    while state != TuiState::Exit {
        match state {
            TuiState::Greeting => greeting(writer)?,
            TuiState::AddRoom => add_room(writer, home)?,
            TuiState::RoomManagement => handle_rooms(writer, home)?,
            TuiState::Report => println!("{}", home.report_to_string()),
            _ => wrong_command(writer)?,
        }

        writeln!(
            writer,
            "{}",
            [
                "Posible actions:",
                "1 - Add a room",
                "2 - Select a room",
                "3 - Report",
                "4 - Exit",
            ]
            .join("\n")
        )?;

        state = match get_input() {
            Result::Ok(input) => match input.as_ref() {
                "1" => TuiState::AddRoom,
                "2" => TuiState::RoomManagement,
                "3" => TuiState::Report,
                "4" => TuiState::Exit,
                _ => TuiState::WrongCommand,
            },
            _ => TuiState::WrongCommand,
        }
    }

    io::Result::Ok(())
}
