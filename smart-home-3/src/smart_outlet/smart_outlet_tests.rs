use crate::smart_outlet::SmartOutlet;
use crate::sensors::MockedSensor;
use crate::common::{SwitchStatusEnum, Report, PRINT_OFFSET};

fn eq(a: f64, b: f64) -> bool {
    f64::abs(a - b) < f64::EPSILON
}

fn make_instance() -> SmartOutlet {
    SmartOutlet::new(
        "OUTLET".to_string(),
        Box::new(MockedSensor::new(vec![20.0])),
        Box::new(MockedSensor::new(vec![30.0])),
        Box::new(MockedSensor::new(vec![SwitchStatusEnum::On])),
    )
}

#[test]
fn should_calculate_power_units_by_utilizing_sensors() {
    let outlet = make_instance();

    assert!(eq(outlet.get_power_units(), 600.0), "calculated wrong value");
}

#[test]
fn should_return_switch_status_from_sensor() {
    let outlet = make_instance();

    assert_eq!(outlet.get_power_state(), SwitchStatusEnum::On);
}

#[test]
fn should_report_status() {
    let outlet = make_instance();
    let report = outlet.report().join("");

    assert_eq!(report, vec![
        "Outlet: OUTLET".to_string(),
        format!("{}power: On", PRINT_OFFSET),
        format!("{}consumption: 0.6kW", PRINT_OFFSET),
    ].join(""));
}
