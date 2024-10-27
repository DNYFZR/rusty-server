// TCP Server
use std::io::{Read, Write};
use std::net;

pub fn run(address:&str) {
    // Listener Server
    let listener = net::TcpListener::bind(address)
        .expect("could not bind to provided address");
  
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("success : {:#?}", stream);
                
                client_handler(stream);
            },

            Err(e) => {
                eprintln!("failed : {e}");
            },
        }
    }
}

fn client_handler(mut stream: net::TcpStream) {
    // Outbound Server
    let mut byte_stream = [0; 1024];
    loop{
        let byte_read = stream.read(&mut byte_stream).expect("failed to read client");
    
        if byte_read == 0 {
            return;
        }

        stream.write_all(&byte_stream[0..byte_read]).expect("failed to write to client");
    }
}