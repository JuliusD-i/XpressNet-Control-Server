mod message_parser;
mod railway_state;
mod serial_communication;

use crate::message_parser::MessageParser;
use crate::railway_state::RailwaySetupState;
use crate::serial_communication::XpressNetInterface;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

const MAXIMUM_BUFFER_SIZE: usize = 10000;

fn main() {
    let mut receive_queue: VecDeque<Vec<u8>> = VecDeque::with_capacity(MAXIMUM_BUFFER_SIZE);
    let mut send_queue: VecDeque<Vec<u8>> = VecDeque::with_capacity(MAXIMUM_BUFFER_SIZE);
    let serial_port = XpressNetInterface::new("/dev/ttyUSB0");
    let railway_state = Rc::new(RefCell::new(RailwaySetupState::new())); // TODO: Wrap this in something threadsafe
    let parser = MessageParser::new(railway_state.clone());

    /*
    // Replace with your actual serial port and baud rate
    let serial_port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;

    // Shared state
    let shared_data = Arc::new(Mutex::new(SharedData::new()));

    // Channel for sending data to be transmitted via serial
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

    // Clone the Arc to move into the thread
    let shared_data_clone = Arc::clone(&shared_data);

    // Serial port reading thread
    thread::spawn(move || {
        let mut port = match serialport::new(serial_port_name, baud_rate).open() {
            Ok(port) => port,
            Err(_) => return -1,
        };

        let mut buffer: Vec<u8> = vec![0; 1024];

        loop {
            match port.read(buffer.as_mut_slice()) {
                Ok(bytes_read) => {
                    let received_data = buffer[..bytes_read].to_vec();
                    if let Ok(mut data) = shared_data_clone.lock() {
                        data.update(received_data.clone());
                    }
                    println!("Received: {:?}", received_data);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                Err(e) => eprintln!("Serial port error: {:?}", e),
            }
        }
    });

    // Serial port writing thread
    /*thread::spawn(move || {
        let settings = SerialPortSettings {
            baud_rate: baud_rate,
            timeout: Duration::from_millis(10),
            ..Default::default()
        };

        let mut port = serialport::open_with_settings(&serial_port_name, &settings)
            .expect("Failed to open serial port");

        loop {
            if let Ok(data_to_send) = rx.recv() {
                if let Err(e) = port.write_all(&data_to_send) {
                    eprintln!("Failed to write to serial port: {:?}", e);
                } else {
                    println!("Sent: {:?}", data_to_send);
                }
            }
        }
    });*/

    // Simulate other threads accessing the shared data and sending data
    thread::sleep(Duration::from_secs(2));
    {
        let data = shared_data.lock().unwrap();
        println!(
            "Main Thread - Last Received Data: {:?}",
            data.get_last_received()
        );
    }

    tx.send(vec![0x41, 0x42, 0x43]).unwrap(); // Send 'ABC' via serial

    // Keep the main thread alive to allow the spawned threads to keep running
    loop {
        thread::sleep(Duration::from_secs(10));
    }

     */
    println!("Hello World!")
}
