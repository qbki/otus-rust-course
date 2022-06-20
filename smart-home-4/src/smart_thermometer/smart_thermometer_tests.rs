use crate::common::{Report, PRINT_OFFSET};
use crate::smart_thermometer::SmartThermometer;

#[test]
fn should_report_status() {
    let outlet = SmartThermometer::new("THERMOMETER").set_temperature(30.0);

    let report = outlet.report().join("");

    assert_eq!(
        report,
        vec![
            "Thermometer: THERMOMETER".to_string(),
            format!("{}temperature: 30.0°C", PRINT_OFFSET),
        ]
        .join("")
    );
}
