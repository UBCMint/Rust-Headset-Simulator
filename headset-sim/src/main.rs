use rand::Rng;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use serde_json::to_string;

/// WebSocket server
fn main () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let random_numbers: Vec<i32> = (0..64).map(|_| rand::thread_rng().gen_range(1..101)).collect();
                let serialized_numbers = to_string(&random_numbers).unwrap();
                websocket.send(tungstenite::Message::Text(serialized_numbers)).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}
