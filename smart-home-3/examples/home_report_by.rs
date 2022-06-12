extern crate smart_home_3 as smart;

use smart::common::ReportType::*;
use smart::mocks::{make_home, BASEMENT, UNKNOWN_OUTLET};

fn main() {
    let home = make_home();

    println!("*** Home ***");
    println!("{}", home.report_by(&Home));
    println!("*** Room ***");
    println!("{}", home.report_by(&Room(BASEMENT)));
    println!("*** Device ***");
    println!("{}", home.report_by(&Device(BASEMENT, UNKNOWN_OUTLET)));
}
