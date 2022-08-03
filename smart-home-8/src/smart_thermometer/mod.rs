use crate::accessors;
use crate::common::{DeviceInterface, Report, POLLING_TIMEOUT, PRINT_OFFSET};
use std::cell::Cell;
use std::sync::Arc;
use std::thread;
use std::time;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

const GET_TEMPERATURE: u8 = 1;

pub struct SmartThermometer {
    name: String,
    address: String,
    temperature: Cell<f64>,
}

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

    accessors!(get_temperature, set_temperature, temperature, f64);

    pub async fn runner(&self) {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .unwrap_or_else(|_| panic!("Can't connect to a thermometer ({})", self.get_name()));
        let socket = Arc::new(socket);

        loop {
            let address = self.get_address();
            let socket = Arc::clone(&socket);
            let temperature = Arc::new(Mutex::new(0.0_f64));
            let inner_temperature = Arc::clone(&temperature);

            tokio::spawn(async move {
                let mut buf = [0; 8];
                socket
                    .send_to(&GET_TEMPERATURE.to_le_bytes(), address)
                    .await
                    .unwrap();
                socket.recv_from(&mut buf).await.unwrap();
                *inner_temperature.lock().await = f64::from_le_bytes(buf);
                thread::sleep(time::Duration::from_millis(POLLING_TIMEOUT));
            })
            .await
            .unwrap();

            self.set_temperature(*temperature.lock().await);
        }
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
