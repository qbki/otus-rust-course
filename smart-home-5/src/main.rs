mod common;
mod mocks;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;
mod tui;

use mocks::make_home;
use tui::run_tui;

fn main() {
    let mut home = make_home();

    run_tui(&mut home).unwrap();
}
