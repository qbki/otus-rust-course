mod common;
mod mocks;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;

use common::RequestType;
use mocks::{make_home, KITCHEN};
use smart_home::Response;

fn main() {
    let mut home = make_home();

    let room = home.get(&RequestType::Room("unknown-room"));
    if let Response(Err(error)) = room {
        println!("{}", error);
    }

    let device = home.get(&RequestType::Device(KITCHEN, "unknown-device"));
    if let Response(Err(error)) = device {
        println!("{}", error);
    }
}
