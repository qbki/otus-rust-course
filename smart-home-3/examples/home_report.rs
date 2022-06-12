extern crate smart_home_3 as smart;

use smart::common::{Report, SwitchStatusEnum::*};
use smart::smart_home::SmartHome;
use smart::smart_outlet::SmartOutlet;
use smart::smart_thermometer::SmartThermometer;

const KITCHEN: &str = "Kitchen";
const LIVING_ROOM: &str = "Living room";
const BASEMENT: &str = "Deep scary basement";

fn main() {
    let fridge_outlet = SmartOutlet::new("Fridge").set_power(2000.0).set_switch(On);
    let unknown_outlet = SmartOutlet::new("Unknown outlet")
        .set_power(1000.0)
        .set_switch(Off);
    let unknown_thermometer = SmartThermometer::new("Unknown thermometer").set_temperature(-10.0);
    let outside_thermometer = SmartThermometer::new("Outside").set_temperature(30.0);
    let inside_thermometer = SmartThermometer::new("Inside").set_temperature(23.0);

    let mut home = SmartHome::new("Home, sweet home");
    home.add_device(KITCHEN, Box::new(fridge_outlet));
    home.add_device(LIVING_ROOM, Box::new(inside_thermometer));
    home.add_device(LIVING_ROOM, Box::new(outside_thermometer));
    home.add_device(BASEMENT, Box::new(unknown_outlet));
    home.add_device(BASEMENT, Box::new(unknown_thermometer));

    println!("{}", home.report().join("\n"));
}
