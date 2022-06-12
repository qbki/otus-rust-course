extern crate smart_home_3 as smart;

use smart::common::Report;
use smart::mocks::make_home;

fn main() {
    let home = make_home();

    println!("{}", home.report().join("\n"));
}
