use crate::accessors;
use crate::common::{DeviceInterface, Report, SwitchStatusEnum, POLLING_TIMEOUT, PRINT_OFFSET};
use std::cell::Cell;
use std::sync::Arc;
use std::thread;
use std::time;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::spawn;
use tokio::sync::Mutex;

const SWITCH_ON: u8 = 1;
const SWITCH_OFF: u8 = 2;
const GET_POWER: u8 = 3;

pub struct SmartOutlet {
    name: String,
    address: String,
    power: Cell<f64>, // Power units (Watt)
    switch: Cell<SwitchStatusEnum>,
}

impl SmartOutlet {
    pub fn new(name: &str, address: &str) -> Self {
        Self {
            name: name.to_string(),
            power: Cell::new(f64::default()),
            switch: Cell::new(SwitchStatusEnum::default()),
            address: address.to_string(),
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    accessors!(get_power, set_power, power, f64);

    accessors!(get_switch, set_switch, switch, SwitchStatusEnum);

    pub async fn runner(&self) {
        let stream = TcpStream::connect(self.get_address())
            .await
            .unwrap_or_else(|_| panic!("Can't update an outlet data ({})", self.get_address()));

        let stream = Arc::new(Mutex::new(stream));
        let power_level = Arc::new(Mutex::new(0.0_f64));

        loop {
            let switch = self.get_switch();
            let stream = Arc::clone(&stream);
            let inner_power_level = Arc::clone(&power_level);

            spawn(async move {
                let mut stream = stream.lock().await;
                match switch {
                    SwitchStatusEnum::On => {
                        stream.write_all(&SWITCH_ON.to_le_bytes()).await.unwrap();
                    }
                    SwitchStatusEnum::Off => {
                        stream.write_all(&SWITCH_OFF.to_le_bytes()).await.unwrap();
                    }
                }

                let mut buf = [0; 8];
                stream.write_all(&GET_POWER.to_le_bytes()).await.unwrap();
                stream.read_exact(&mut buf).await.unwrap();
                *inner_power_level.lock().await = f64::from_le_bytes(buf);

                thread::sleep(time::Duration::from_millis(POLLING_TIMEOUT));
            })
            .await
            .unwrap();

            self.set_power(*power_level.lock().await);
        }
    }
}

impl DeviceInterface for SmartOutlet {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Report for SmartOutlet {
    fn report(&self) -> Vec<String> {
        vec![
            format!("Outlet: {}", self.get_name()),
            format!("{}switch: {}", PRINT_OFFSET, self.get_switch()),
            format!(
                "{}consumption: {:.1}kW",
                PRINT_OFFSET,
                self.get_power() * 0.001
            ),
        ]
    }
}
