use crate::smart_thermometer::SmartThermometer;
use crate::sensors::MockedSensor;
use crate::common::{
    PRINT_OFFSET,
    Report,
    eq_floats,
};

fn make_instance() -> SmartThermometer {
    SmartThermometer::new(
        "THERMOMETER".to_string(),
        Box::new(MockedSensor::new(vec![30.0])),
    )
}

#[test]
fn should_return_temperature_from_sensor() {
    let thermometer = make_instance();

    assert!(eq_floats(thermometer.get_temperature(), 30.0));
}

#[test]
fn should_report_status() {
    let outlet = make_instance();
    let report = outlet.report().join("");

    assert_eq!(report, vec![
        "Thermometer: THERMOMETER".to_string(),
        format!("{}temperature: 30.0°C", PRINT_OFFSET),
    ].join(""));
}
