use std::io::{Read, Write};
use std::net;
use std::sync::{Arc, Mutex};
use std::thread;

const SWITCH_ON: u8 = 1;
const SWITCH_OFF: u8 = 2;
const SEND_POWER: u8 = 3;

fn increment(counter: &Arc<Mutex<usize>>, array: &Vec<f64>) -> f64 {
    let mut counter_value = counter.lock().unwrap();
    let value = array[*counter_value % array.len()];
    *counter_value += 1;
    value
}

fn main() {
    let tcp_addresses = vec!["127.0.0.1:20001"];
    let power_levels = Arc::new(vec![
        1900.0, 2800.0, 3700.0, 4600.0, 5500.0, 6400.0, 7300.0, 8200.0, 9100.0,
    ]);
    let power_level_counter = Arc::new(Mutex::new(0));

    let tcp_threads: Vec<thread::JoinHandle<thread::Result<()>>> = tcp_addresses
        .into_iter()
        .map(move |address| -> thread::JoinHandle<thread::Result<()>> {
            let counter = Arc::clone(&power_level_counter);
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
                                println!("Turn off an outlet ({})", &address);
                            }
                            SEND_POWER => {
                                println!("Send power level ({})", &address);
                                let power_level = if is_outlet_on {
                                    increment(&counter, &power_levels)
                                } else {
                                    0.0
                                };
                                stream.write_all(&f64::to_le_bytes(power_level)).unwrap();
                            }
                            _ => println!("Unknown command"),
                        };
                    }
                }
                Ok(())
            })
        })
        .collect();

    for thread in tcp_threads {
        thread.join().unwrap().unwrap();
    }
}
