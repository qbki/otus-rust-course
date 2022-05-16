pub mod smart_outlet;
pub mod smart_thermometer;
pub mod sensors;

use std::thread::sleep;
use std::time::Duration;
use smart_outlet::SmartOutlet;
use smart_thermometer::SmartThermometer;
use sensors::{MockedSensor, Sensor};

fn main() {
    let mut outlet = SmartOutlet::new(String::from("Kitchen"));
    let mut termometer = SmartThermometer::new(String::from("Outside"));

    let mocked_amperage_sensor = MockedSensor::new(vec![1.0, 3.0, 4.0]);
    let mocked_voltage_sensor = MockedSensor::new(vec![1.0, 3.0, 4.0]);
    let mocked_temperature_sensor = MockedSensor::new(vec![23.0, 24.0, 30.0]);

    let sleep_duration = Duration::from_millis(1000);
    loop {
        let amperage = mocked_amperage_sensor.sample();
        let voltage = mocked_voltage_sensor.sample();
        let temperature = mocked_temperature_sensor.sample();

        outlet.set_power_consumption(voltage, amperage);
        termometer.set_temperature(temperature);

        print!("\x1B[2J"); // clear screen
        print!("\x1B[H"); // move cursor to (0, 0)
        println!("{}, {:#?}", outlet.description(), outlet.power_state());
        println!("{}, {}", termometer.description(), termometer.get_temperature());

        sleep(sleep_duration);
    }
}
