mod common;
mod mocks;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;

use common::Report;
use mocks::make_home;
use std::thread;
use std::time::Duration;

fn main() {
    let home = make_home();

    home.run();
    loop {
        print!("\x1B[2J"); // clear screen
        print!("\x1B[H"); // move cursor to (0, 0)
        println!("{}", home.report_to_string());
        thread::sleep(Duration::from_millis(1000));
    }
}
