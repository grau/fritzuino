use log::{info, debug};
use std::thread::sleep;
use std::time::Duration;
use pretty_env_logger;

mod cli_parse;
mod sip_call;
mod wait_ardu;

/// Main entry point
/// Next: Add https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
/// For command line argument parsing.
fn main() {
    let (call_data, serial_port, duration, log_level) = cli_parse::parse_parameters();
    pretty_env_logger::formatted_timed_builder()
        .filter(None, log_level)
        .init();
    debug!("Verbosity level: {:?}", log_level);
    
    if serial_port == "" {
        info!("Performing SIP call");
        sip_call::call(&call_data, duration)
                .expect("SIP call failed!");
    } else {
        info!("Application started. Waiting for RING on {}", serial_port);
        loop {
            wait_ardu::wait_for_ardu(String::from(&serial_port));
            sip_call::call(&call_data, duration)
                .expect("SIP call failed!");
            sleep(Duration::from_millis(5000));
        }
    }
}

