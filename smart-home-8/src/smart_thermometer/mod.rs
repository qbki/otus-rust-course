use std::cell::Cell;
use std::sync::Arc;
use std::thread;
use std::time;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use crate::common::{DeviceInterface, Report, POLLING_TIMEOUT, PRINT_OFFSET};

const GET_TEMPERATURE: u8 = 1;

#[derive(Clone)]
pub struct SmartThermometer {
    name: String,
    address: String,
    temperature: Cell<f64>, }

impl SmartThermometer {
    pub fn new(name: &str, address: &str) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
            temperature: Cell::new(f64::default()),
        }
    }

    pub fn get_address(&self) -> String {
        self.address.clone()
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature.get()
    }

    pub fn set_temperature(&self, value: f64) {
        self.temperature.set(value);
    }

    pub async fn runner(&self) {
        let socket_result = UdpSocket::bind("0.0.0.0:0").await;
        match socket_result {
            Ok(socket) => {
                let socket = &Arc::new(socket);
                loop {
                    let address = self.get_address();
                    let socket = Arc::clone(&socket);
                    let new_temperature = &Arc::new(Mutex::new(Cell::new(0.0 as f64)));
                    let cloned_new_temperature = Arc::clone(&new_temperature);
                    tokio::spawn(async move {
                        thread::sleep(time::Duration::from_millis(POLLING_TIMEOUT));
                        let mut buf = [0; 8];
                        socket
                            .send_to(&GET_TEMPERATURE.to_le_bytes(), address)
                            .await
                            .unwrap();
                        socket
                            .recv_from(&mut buf)
                            .await
                            .unwrap();
                        let temperature = f64::from_le_bytes(buf);
                        cloned_new_temperature.lock().await.set(temperature);
                    }).await.unwrap();
                    self.set_temperature(new_temperature.lock().await.get());
                }
            },
            Err(_) => {
                eprintln!("Can't connect to a thermometer ({})", self.get_name());
            }
        };
    }
}

impl DeviceInterface for SmartThermometer {
    fn get_name(&self) -> String {
        self.name.clone()
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
