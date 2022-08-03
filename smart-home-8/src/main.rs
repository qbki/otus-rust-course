mod common;
mod mocks;
mod smart_home;
mod smart_outlet;
mod smart_room;
mod smart_thermometer;

use common::{Report, UI_UPDATE_TIMEOUT};
use mocks::make_home;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let home = make_home();

    tokio::join!(home.run(), async {
        loop {
            let report = home.report_to_string();
            tokio::spawn(async move {
                print!("\x1B[2J"); // clear screen
                print!("\x1B[H"); // move cursor to (0, 0)
                println!("{}", report);
                thread::sleep(Duration::from_millis(UI_UPDATE_TIMEOUT));
            })
            .await
            .unwrap();
        }
    },);
}
