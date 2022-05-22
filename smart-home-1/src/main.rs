mod common;
mod sensors;
mod smart_outlet;
mod smart_thermometer;

use common::SwitchStatusEnum;
use sensors::{MockedSensor, Sensor};
use smart_outlet::SmartOutlet;
use smart_thermometer::SmartThermometer;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut outlet = SmartOutlet::new(String::from("Kitchen"));
    let mut thermometer = SmartThermometer::new(String::from("Outside"));

    let mocked_amperage_sensor = MockedSensor::new(vec![1000.0, 2000.0, 3000.0]);
    let mocked_voltage_sensor = MockedSensor::new(vec![4000.0, 5000.0, 6000.0]);
    let mocked_temperature_sensor = MockedSensor::new(vec![23.0, 24.0, 30.0]);
    let mocked_switch_state = MockedSensor::new(vec![
        SwitchStatusEnum::On,
        SwitchStatusEnum::Off,
        SwitchStatusEnum::On,
    ]);

    let sleep_duration = Duration::from_millis(1000);
    loop {
        let switch_state = mocked_switch_state.sample();
        let (amperage, voltage) = if let SwitchStatusEnum::On = switch_state {
            (
                mocked_amperage_sensor.sample(),
                mocked_voltage_sensor.sample(),
            )
        } else {
            (0.0, 0.0)
        };
        let temperature = mocked_temperature_sensor.sample();

        outlet.set_power_consumption(voltage, amperage);
        outlet.set_power_state(switch_state);
        thermometer.set_temperature(temperature);

        print!("\x1B[2J"); // clear screen
        print!("\x1B[H"); // move cursor to (0, 0)
        println!("{}", outlet);
        println!("{}", thermometer);

        sleep(sleep_duration);
    }
}
