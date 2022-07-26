use std::net;
use std::thread;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

const SWITCH_ON: u8 = 1;
const SWITCH_OFF: u8 = 2;
const SEND_POWER: u8 = 3;

fn main() {
    let tcp_addresses = vec!["127.0.0.1:20001", "127.0.0.1:20002"];
    let power_levels = Arc::new(vec![1900.0, 2800.0, 3700.0, 4600.0, 5500.0, 6400.0, 7300.0, 8200.0, 9100.0]);
    let counter = Arc::new(Mutex::new(0));


    let tcp_threads: Vec<thread::JoinHandle<thread::Result<()>>> = tcp_addresses 
        .into_iter()
        .map(|address| -> thread::JoinHandle<thread::Result<()>> {
            let counter = Arc::clone(&counter);
            let power_levels = Arc::clone(&power_levels);

            thread::spawn(move || -> thread::Result<()> {
                let listener = net::TcpListener::bind(address).unwrap();
                println!("Bound to: {}", &address);

                for stream_result in listener.incoming() {
                    let mut buf = [0; 1];
                    let mut stream = stream_result.unwrap();
                    let mut is_outlet_on = false;
                    loop {
                        stream.read_exact(&mut buf).unwrap();
                        match u8::from_le_bytes(buf) {
                            SWITCH_ON => {
                                is_outlet_on = true;
                                println!("Turn on an outlet ({})", &address);
                            }
                            SWITCH_OFF => {
                                is_outlet_on = false;
                                println!("Turn off an outlet ({})",&address);
                            }
                            SEND_POWER => {
                                println!("Send power level ({})",&address);
                                let power_level = if is_outlet_on {
                                    let mut counter_value = counter.lock().unwrap();
                                    let value = power_levels[*counter_value % power_levels.len()];
                                    *counter_value += 1;
                                    value
                                } else {
                                    0.0
                                };
                                stream.write(&f64::to_le_bytes(power_level)).unwrap();
                            },
                            _ => println!("Unknown command"),
                        };
                    }
                };
                Ok(())
            })
        })
        .collect();

    for thread in tcp_threads.into_iter() {
        thread.join().unwrap().unwrap();
    }
}
