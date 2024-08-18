use serialport::{DataBits, Parity, SerialPort, StopBits};
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Debug)]
pub struct XpressNetInterface {
    port: Box<dyn SerialPort>,
    my_bus_id: u8,
}

impl XpressNetInterface {
    pub fn new(port_name: &str) -> Self {
        let serial_port = serialport::new(port_name, 62500)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .timeout(Duration::from_millis(100));
        XpressNetInterface {
            port: serial_port.open().expect("Error opening serial port"),
            my_bus_id: 0,
        }
    }

    pub fn run(receive_queue: &VecDeque<Vec<u8>>, send_queue: &VecDeque<Vec<u8>>) {


    }
}
