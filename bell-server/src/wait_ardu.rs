use serialport;
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead;
use std::io::ErrorKind;
use log::{info,debug,warn};

/// Outputs a list of all available serial ports to console
// pub fn show_ports() {
//     let ports = serialport::available_ports()
//         .expect("Could not fest serial ports");
//     for (index, port) in ports.iter().enumerate() {
//         println!("({}) Port name {}, port type: {:?}",
//             index, port.port_name, port.port_type);
//     }
// }

/// Reads line from serial console. Returns once it found the string "RING".
///
/// # Arguments
/// * `serial_port` - Name of serial port to connect to
pub fn wait_for_ardu(serial_port: String) {
    let serial_port = serialport::new(serial_port, 115200)
        .timeout(Duration::from_millis(5000))
        .open()
        .expect("Failed to open serial port");

    let mut reader = BufReader::new(serial_port);
    let mut ardu_response = String::new();
    loop {
        ardu_response.clear();
        reader.read_line(&mut ardu_response).unwrap_or_else(|error| {
            match error.kind() {
                ErrorKind::TimedOut => {
                    warn!("REACHED TIMEOUT!");
                    return 0;
                }
                ErrorKind::InvalidData => {
                    debug!("Invalid data. Just ignore");
                    return 0;
                }
                _ => {
                    panic!("Critical read failure: {}", error.kind());
                }
            }
        });
        if ardu_response.trim().eq("RING") {
            info!("Button press detected");
            return;
        }
    }
}