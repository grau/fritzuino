/// Module for performing simple sip calls

use std::io::prelude::*;
use std::net::TcpStream;
use std::{str, thread, time};
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;
use log::{debug,info};

/// Data structure for initiating a sip call
pub struct SipCallData {
    pub source: String,
    pub server: String,
    pub port: i32,
    pub target: String,
    pub username: String,
    pub password: String,
}

/// Initiates a sip call
/// 
/// # Arguments
/// * `call_data` - SIP data for call
/// * `sleep_time` - Duration to keep it ringing
pub fn call(call_data: &SipCallData, sleep_time: u64) -> std::io::Result<()> {
    info!("Starting SIP-Call to {}", call_data.target);
    let uri = ["sip:", &call_data.username, "@", &call_data.server].concat();

    // This is ugly - but I don't yet understand how to use "connect(server, port)". There is some ... ownership problem?
    let mut stream = TcpStream::connect([&call_data.server, ":", &call_data.port.to_string()].concat())
        .expect("Failed to open tcp stream");

    let call_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();

    let default_header = get_default_sip_header(&call_data, &call_id);
    let auth = retrieve_realm_nonce(&mut stream, &default_header);
    let digest = compute_digest(&call_data, &uri, &auth);
    perform_call(&mut stream, &default_header, &call_data, &auth, &digest, &uri);
    debug!("Sleeping {} ms", sleep_time);
    thread::sleep(time::Duration::from_millis(sleep_time));
    perform_cancel(&mut stream, &default_header);
    info!("SIP-Call finished");
    Ok(())
}

/// Default SIP header for all calls with some base infos
/// 
/// # Arguments
/// * `call_data` - SIP data for call
/// * `call_id` - Some random text (timestamp in this case) as unique call-id
fn get_default_sip_header(call_data: &SipCallData, call_id: &String) -> String {
    return ["sip: ", &call_data.server, " SIP/2.0\n",
        "From: <sip:", &call_data.username, "@", &call_data.source, ">\n",   // Replace by local IP!
        "Via: SIP/2.0/TCP ", &call_data.server, "\n",
        "To: <sip:", &call_data.target, ">;tag=x\n",
        "Call-ID: i", &call_id ,"\n",
        "User-Agent: Stupid rust script\n",
        "Content-Length: 0\n"].concat();
}


/// Sends a first SIP invite request.
/// This will be denied with a 401 and realm as well as nonce. Those two are returned.
/// 
/// # Arguments
/// * `stream` - TCP stream to use for sending data
/// * `default_header` - Base SIP call header
fn retrieve_realm_nonce(stream: &mut std::net::TcpStream, default_header: &String) -> (String, String) {
    debug!("=======First request========");
    let invite_request_1 = ["INVITE ", &default_header,
    "Cseq: 1 INVITE\n",
    "\n\n"].concat();
    debug!("Request:\n{}", invite_request_1);
    stream.write(invite_request_1.as_bytes())
        .expect("Failed to write first invite");
    let mut response: [u8; 2048] = [0; 2048];
    stream.read(&mut response)
        .expect("Failed to read first invite response");
    let response_string = match str::from_utf8(&response) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    debug!("Response:\n{}", response_string);
    

    let regex_string = "realm=\"(?P<realm>.+)\", nonce=\"(?P<nonce>[A-F0-9]+)\"";
    let re_nonce = Regex::new(regex_string)
        .expect("Failed to compile regex");
    let captures = re_nonce.captures(response_string)
        .expect("Failed to apply regex");
    let realm = captures.name("realm").expect("No realm found").as_str().to_owned();
    let nonce = captures.name("nonce").expect("No nonce found").as_str().to_owned();

    debug!("Got realm: {}, nonce: {}", realm, nonce);

    return (realm, nonce);
}

/// Sends a second SIP invite request.
/// This time with valid authentification data. This will start the ringing!
/// 
/// # Arguments
/// * `stream` - TCP stream to use for sending data
/// * `default_header` - Base SIP call header
/// * `call_data` - SIP data for call
/// * `auth` - Tuple of (realm,nonce) - connection auth data
/// * `digest` - Calculated md5 authentification digest
fn perform_call(stream: &mut std::net::TcpStream, default_header: &String, call_data: &SipCallData, auth: &(String, String), digest: &String, uri: &String) {
    debug!("=======Second request========");
    let invite_request_2 = ["INVITE ", &default_header,
        "Authorization: Digest username=\"", &call_data.username, "\",\n",
        " realm=\"", &auth.0, "\",\n",
        " nonce=\"", &auth.1, "\",\n",
        " uri=\"", &uri, "\",\n",
        " response=\"", &digest, "\"\n",
        "Cseq: 2 INVITE\n",
        "\n\n"].concat();
    debug!("Request:\n{}", invite_request_2);
    stream.write(invite_request_2.as_bytes())
        .expect("Failed to write second invite");
    let mut response: [u8; 2048] = [0; 2048];
    stream.read(&mut response)
        .expect("Failed to read second invite response");
    let response_string = match str::from_utf8(&response) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    debug!("Response:\n{}", response_string);
}

/// Sends a cancel request.
/// This will stop the ringing!
/// 
/// # Arguments
/// * `stream` - TCP stream to use for sending data
/// * `default_header` - Base SIP call header
fn perform_cancel(stream: &mut std::net::TcpStream, default_header: &String) {
    debug!("=======Cancel request========");
    let cancel_request = ["CANCEL ", &default_header,
        "Cseq: 2 CANCEL\n",
        "\n\n"].concat();
    debug!("Request:\n{}", cancel_request);
    stream.write(cancel_request.as_bytes())
        .expect("Failed to send cancel");
    let mut response: [u8; 2048] = [0; 2048];
    stream.read(&mut response)
        .expect("Failed to read second invite response");
    let response_string = match str::from_utf8(&response) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    debug!("Response:\n{}", response_string);
}

/// Computes md5 digest for authentification
/// 
/// # Arguments
/// * `call_data` - SIP data for call
/// * `uri` - URI to connect to
/// * `auth` - Tuple of (realm,nonce) - connection auth data
fn compute_digest(call_data: &SipCallData, uri: &String, auth: &(String, String)) -> String {
    let a1_val = md5::compute([&call_data.username, ":", &auth.0, ":", &call_data.password].concat());
    let a1 = format!("{:x}", a1_val);
    let a2 = format!("{:x}", md5::compute(["INVITE:", &uri].concat()));
    let digest = format!("{:x}", md5::compute([&a1, ":", &auth.1, ":", &a2].concat()));
    return digest;
}
