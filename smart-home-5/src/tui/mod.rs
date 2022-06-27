mod utils;
mod device_management;
mod home_management;
mod room_management;

use std::io;
use crate::smart_home::SmartHome;
use home_management::home_management;


pub fn run_tui(home: &mut SmartHome) -> io::Result<()> {
    let mut writer = io::stdout();

    home_management(&mut writer, home)
}
