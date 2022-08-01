use crate::common::{DeviceInterface, Report, POLLING_TIMEOUT, PRINT_OFFSET};
use std::cell::Cell;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

const GET_TEMPERATURE: u8 = 1;

#[derive(Clone)]
pub struct SmartThermometerData {
    name: String,
    address: String,
    temperature: Cell<f64>,
}

pub struct SmartThermometer(Arc<Mutex<SmartThermometerData>>);

impl SmartThermometer {
    pub fn new(name: &str, address: &str) -> Self {
        Self(Arc::new(Mutex::new(SmartThermometerData {
            name: name.to_string(),
            address: address.to_string(),
            temperature: Cell::new(f64::default()),
        })))
    }

    pub fn get_address(&self) -> String {
        self.0.lock().unwrap().address.clone()
    }

    pub fn get_temperature(&self) -> f64 {
        self.0.lock().unwrap().temperature.get()
    }

    pub fn set_temperature(&self, value: f64) {
        self.0.lock().unwrap().temperature.set(value);
    }

    pub fn runner(&self) {
        let inner = Arc::clone(&self.0);
        thread::spawn(move || {
            // Let OS deside what port to use
            let socket_result = UdpSocket::bind("0.0.0.0:0");
            match socket_result {
                Ok(socket) => loop {
                    let mut buf = [0; 8];
                    thread::sleep(time::Duration::from_millis(POLLING_TIMEOUT));
                    let inner = inner.lock().unwrap();

                    socket
                        .send_to(&GET_TEMPERATURE.to_le_bytes(), inner.address.clone())
                        .unwrap();
                    socket.recv_from(&mut buf).unwrap();
                    let temperature = f64::from_le_bytes(buf);
                    inner.temperature.set(temperature);
                },
                Err(_) => {
                    eprintln!("Can't update temperature ({})", inner.lock().unwrap().name);
                }
            };
        });
    }
}

impl DeviceInterface for SmartThermometer {
    fn get_name(&self) -> String {
        self.0.lock().unwrap().name.clone()
    }
}

impl Report for SmartThermometer {
    fn report(&self) -> Vec<String> {
        vec![
            format!("Thermometer: {}", self.get_name()),
            format!(
                "{}temperature: {:.1}°C",
                PRINT_OFFSET,
                self.get_temperature()
            ),
        ]
    }
}
