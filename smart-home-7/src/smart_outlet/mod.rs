use crate::common::{DeviceInterface, Report, SwitchStatusEnum, POLLING_TIMEOUT, PRINT_OFFSET};
use std::cell::Cell;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

const SWITCH_ON: u8 = 1;
const SWITCH_OFF: u8 = 2;
const GET_POWER: u8 = 3;

struct SmartOutletData {
    name: String,
    address: String,
    power: Cell<f64>, // Power units (Watt)
    switch: Cell<SwitchStatusEnum>,
}

pub struct SmartOutlet(Arc<Mutex<SmartOutletData>>);

impl SmartOutlet {
    pub fn new(name: &str, address: &str) -> Self {
        Self(Arc::new(Mutex::new(SmartOutletData {
            name: name.to_string(),
            power: Cell::new(f64::default()),
            switch: Cell::new(SwitchStatusEnum::default()),
            address: address.to_string(),
        })))
    }

    pub fn get_address(&self) -> String {
        self.0.lock().unwrap().address.clone()
    }

    pub fn get_power(&self) -> f64 {
        self.0.lock().unwrap().power.get()
    }

    pub fn set_power(&self, value: f64) {
        self.0.lock().unwrap().power.set(value);
    }

    pub fn get_switch(&self) -> SwitchStatusEnum {
        self.0.lock().unwrap().switch.get()
    }

    pub fn set_switch(&self, value: SwitchStatusEnum) {
        self.0.lock().unwrap().switch.set(value);
    }

    pub fn runner(&self) {
        let inner = Arc::clone(&self.0);
        thread::spawn(move || {
            let connection = {
                let inner = inner.lock().unwrap();
                TcpStream::connect(inner.address.clone())
            };
            match connection {
                Ok(mut stream) => loop {
                    let mut buf = [0; 8];
                    thread::sleep(time::Duration::from_millis(POLLING_TIMEOUT));
                    let inner = inner.lock().unwrap();

                    let switch = inner.switch.get();
                    match switch {
                        SwitchStatusEnum::On => {
                            stream.write_all(&SWITCH_ON.to_le_bytes()).unwrap();
                        }
                        SwitchStatusEnum::Off => {
                            stream.write_all(&SWITCH_OFF.to_le_bytes()).unwrap();
                        }
                    }

                    stream.write_all(&GET_POWER.to_le_bytes()).unwrap();
                    stream.read_exact(&mut buf).unwrap();
                    let power = f64::from_le_bytes(buf);
                    inner.power.set(power);
                },
                Err(_) => {
                    eprintln!(
                        "Can't update an outlet data ({})",
                        inner.lock().unwrap().name
                    );
                }
            };
        });
    }
}

impl DeviceInterface for SmartOutlet {
    fn get_name(&self) -> String {
        self.0.lock().unwrap().name.clone()
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
