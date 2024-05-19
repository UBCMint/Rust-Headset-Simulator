use rand::Rng;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use serde_json::to_string;
mod createMatrix;
use createMatrix::generate_random_matrix;

/// WebSocket server
fn main () {
    // let random_matrix = generate_random_matrix();
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        let randomMtrx = generate_random_matrix();
        println!("{:?}", randomMtrx);
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                websocket.send(tungstenite::Message::Text(randomMtrx.clone())).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}
