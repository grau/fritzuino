/// Module for command line parsing using clap

use clap::{arg, command, ArgAction};
use log::LevelFilter;
use crate::sip_call::SipCallData;

/// Parses command line parameters.
/// Returns SipCallData, arduino serial port, ring duration and log level (number of -v)
pub fn parse_parameters() -> (SipCallData, String, u64, log::LevelFilter) {
    let matches = command!()
        .arg(arg!(-s --source <SOURCE_IP> "Source IP address").env("BELL_SOURCE_IP"))
        .arg(arg!(-d --server <SERVER_IP> "Destination / SIP server ip address").env("BELL_SERVER_IP"))
        .arg(arg!(-e --port <SERVER_PORT> "TCP Port of SIP Server")
            .required(false)
            .value_parser(parse_port)
            .env("BELL_SERVER_PORT"))
        .arg(arg!(-t --target <SIP_TARGET> "SIP Target address").env("BELL_SIP_TARGET"))
        .arg(arg!(-u --username <USERNAME> "Username for SIP authentification").env("BELL_USERNAME"))
        .arg(arg!(-p --password <PASSWORD> "Password for SIP authentification").env("BELL_PASSWORD"))
        .arg(arg!(-m --duration <DURATION_MILLIS> "Ring duration in milliseconds. Defaults to 2000")
            .required(false)
            .value_parser(|s: &str| s.parse::<u64>())
            .env("BELL_DURATION_MILLIS"))
        .arg(arg!(-r --serial <SERIAL_PORT> "Serial port of arduino. If not provided will ring once and then terminate")
            .required(false).env("BELL_SERIAL"))
        .arg(arg!(-v --verbose "Use verbose output (-vv very verbose/debug)").action(ArgAction::Count))
        .get_matches();

    let call_data = SipCallData {
        source: String::from(matches.get_one::<String>("source").expect("Source required")),
        server: String::from(matches.get_one::<String>("server").expect("Server required")),
        port: matches.get_one::<i32>("port").unwrap_or(&5060).clone(),
        target: String::from(matches.get_one::<String>("target").expect("Target required")),
        username: String::from(matches.get_one::<String>("username").expect("Username required")),
        password: String::from(matches.get_one::<String>("password").expect("Password required")),
    };

    let serial_port = String::from(matches.get_one::<String>("serial").unwrap_or(&String::from("")));

    let duration = matches.get_one::<u64>("duration").unwrap_or(&2000);
    let log_level = match matches.get_one::<u8>("verbose").unwrap() {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Debug
    };

    return (call_data, serial_port, *duration, log_level);
}

/// Parses the given string to a valid port number.
/// Or dies trying
/// 
/// # Arguments
/// * `s` - String to parse
fn parse_port(s: &str) -> Result<i32, String> {
    let port: i32 = s
        .parse()
        .map_err(|_| format!("`{}` isn't a port number", s))?;
    if port < 0 || port > 65535 {
        Err(format!("Port {} not in range 0-65535", port))
    } else {
        Ok(port)
    }
}
