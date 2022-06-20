extern crate smart_home_4 as smart;

use smart::common::RequestType::*;
use smart::mocks::{make_home, BASEMENT, UNKNOWN_OUTLET};

fn main() {
    let home = make_home();

    let home_report: String = home.get(&Home).into();
    let room_report: String = home.get(&Room(BASEMENT)).into();
    let devices_report: String = home.get(&Device(BASEMENT, UNKNOWN_OUTLET)).into();

    println!("*** Home ***");
    println!("{}", home_report);
    println!("*** Room ***");
    println!("{}", room_report);
    println!("*** Device ***");
    println!("{}", devices_report);
}
