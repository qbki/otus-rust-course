use crate::common::{Device, SwitchStatusEnum::*};
use crate::smart_home::SmartHome;
use crate::smart_outlet::SmartOutlet;
use crate::smart_thermometer::SmartThermometer;

pub const KITCHEN: &str = "Kitchen";
pub const LIVING_ROOM: &str = "Living room";
pub const BASEMENT: &str = "Deep scary basement";

pub const UNKNOWN_OUTLET: &str = "Unknown outlet";

pub fn make_home() -> SmartHome {
    let fridge_outlet = SmartOutlet::new("Fridge");
    fridge_outlet.set_power(2000.0).set_switch(On);

    let unknown_outlet = SmartOutlet::new(UNKNOWN_OUTLET);
    unknown_outlet.set_power(1000.0).set_switch(Off);

    let unknown_thermometer = SmartThermometer::new("Unknown thermometer");
    unknown_thermometer.set_temperature(-10.0);

    let outside_thermometer = SmartThermometer::new("Outside");
    outside_thermometer.set_temperature(30.0);

    let inside_thermometer = SmartThermometer::new("Inside");
    inside_thermometer.set_temperature(23.0);

    let mut home = SmartHome::new("Home, sweet home");
    home.add_device(KITCHEN, Device::Outlet(fridge_outlet));
    home.add_device(LIVING_ROOM, Device::Thermometer(inside_thermometer));
    home.add_device(LIVING_ROOM, Device::Thermometer(outside_thermometer));
    home.add_device(BASEMENT, Device::Outlet(unknown_outlet));
    home.add_device(BASEMENT, Device::Thermometer(unknown_thermometer));

    home
}
