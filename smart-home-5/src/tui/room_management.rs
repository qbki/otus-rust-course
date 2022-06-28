use super::device_management::device_management;
use super::utils::{get_input, get_name, wrong_command};
use crate::common::{Device, Report};
use crate::smart_outlet::SmartOutlet;
use crate::smart_room::SmartRoom;
use crate::smart_thermometer::SmartThermometer;
use std::io;

#[derive(PartialEq)]
enum RoomManagement {
    Exit,
    Greeting,
    AddOutlet,
    AddThermometer,
    DeviceManagement,
    WrongCommand,
    Report,
}

fn greeting(writer: &mut dyn io::Write) -> io::Result<()> {
    writeln!(writer, "Room Management")
}

fn add_outlet(writer: &mut dyn io::Write, room: &mut SmartRoom) -> io::Result<()> {
    writeln!(writer, "Please enter an outlet name:")?;

    let name = get_name(writer)?;
    let device = SmartOutlet::new(&name);

    room.add_device(Device::Outlet(device));

    Result::Ok(())
}

fn add_thermometer(writer: &mut dyn io::Write, room: &mut SmartRoom) -> io::Result<()> {
    writeln!(writer, "Please enter a thermometer name:")?;

    let name = get_name(writer)?;
    let device = SmartThermometer::new(&name);

    room.add_device(Device::Thermometer(device));

    Result::Ok(())
}

fn handle_devices(writer: &mut dyn io::Write, room: &mut SmartRoom) -> io::Result<()> {
    writeln!(writer, "Please enter a device name:")?;

    let name = get_name(writer)?;
    let room_result = room.get_device(&name);

    match room_result {
        Some(device) => device_management(writer, device),
        None => writeln!(writer, "Can't find a device"),
    }
}

pub fn room_management(writer: &mut dyn io::Write, room: &mut SmartRoom) -> io::Result<()> {
    let mut state = RoomManagement::Greeting;

    while state != RoomManagement::Exit {
        match state {
            RoomManagement::Greeting => greeting(writer)?,
            RoomManagement::Report => println!("{}", room.report_to_string()),
            RoomManagement::AddOutlet => add_outlet(writer, room)?,
            RoomManagement::AddThermometer => add_thermometer(writer, room)?,
            RoomManagement::DeviceManagement => handle_devices(writer, room)?,
            _ => wrong_command(writer)?,
        }

        writeln!(
            writer,
            "{}",
            [
                "Room management:",
                "1 - Add an outlet",
                "2 - Add a thermometer",
                "3 - Select a device",
                "4 - Report",
                "5 - Back",
            ]
            .join("\n")
        )?;

        state = match get_input() {
            Result::Ok(input) => match input.as_ref() {
                "1" => RoomManagement::AddOutlet,
                "2" => RoomManagement::AddThermometer,
                "3" => RoomManagement::DeviceManagement,
                "4" => RoomManagement::Report,
                "5" => RoomManagement::Exit,
                _ => RoomManagement::WrongCommand,
            },
            _ => RoomManagement::WrongCommand,
        };
    }

    Result::Ok(())
}
