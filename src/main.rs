use std::io::Write;
use std::net::TcpListener;

const SIMPLE_STRING_PREFIX: &str = "+";
const CLRF: &str = "\r\n";
const PING_RESPONSE: &str = "PONG";

fn main() {
    let host = "127.0.0.1";
    let port: u16 = 6379;

    let listener = TcpListener::bind((host, port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("established connection");
                let _ = _stream.write(simple_string_message(PING_RESPONSE).as_bytes());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn simple_string_message(message: &str) -> String {
    format!("{}{}{}", SIMPLE_STRING_PREFIX, message, CLRF)
}
