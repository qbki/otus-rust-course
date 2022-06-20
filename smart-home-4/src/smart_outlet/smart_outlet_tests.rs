use crate::common::{Report, SwitchStatusEnum, PRINT_OFFSET};
use crate::smart_outlet::SmartOutlet;

#[test]
fn should_report_status() {
    let outlet = SmartOutlet::new("OUTLET");
    outlet
        .set_switch(SwitchStatusEnum::On)
        .set_power(600.0);

    let report = outlet.report().join("");

    assert_eq!(
        report,
        vec![
            "Outlet: OUTLET".to_string(),
            format!("{}switch: On", PRINT_OFFSET),
            format!("{}consumption: 0.6kW", PRINT_OFFSET),
        ]
        .join("")
    );
}
