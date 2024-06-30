use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:37")
        .expect("Failed to etablish a TCP listener on: 0.0.0.0:37");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            _ => (),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let local_server_time = SystemTime::now();

    // Adjusting local server time by subtracting 3 hours (3 * 3600 seconds)
    // to convert from UTC +3 timezone (this server) to GMT (UTC ±0) time.
    let gmt_server_time = match local_server_time.checked_sub(Duration::from_secs(3600 * 3)) {
        Some(gmt_t) => gmt_t,
        None => return,
    };

    // Origin time: 1 January 1900 00:00:00 GMT (UTC ±0)
    let origin_time = match UNIX_EPOCH.checked_sub(Duration::from_secs(
        (53 * 365 * 24 * 3600) + (17 * 366 * 24 * 3600),
    )) {
        Some(origin_t) => origin_t,
        None => return,
    };

    let seconds = match gmt_server_time.duration_since(origin_time) {
        Ok(d) => d.as_secs() as u32,
        Err(_) => return,
    };

    match stream.write(format!("{:032b}", seconds).as_bytes()) {
        Ok(_) => (),
        Err(_) => return,
    }
}
