mod common;
mod smart_outlet;

use common::{Report, SwitchStatusEnum};
use smart_outlet::SmartOutlet;

fn main() {
    let mut outlet = SmartOutlet::new("Fridge");
    outlet.set_switch(SwitchStatusEnum::On);

    println!("{}", outlet.report_to_string());
}
