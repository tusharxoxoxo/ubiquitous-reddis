 use std::{net::TcpListener, io::Write};

fn main() {
    println!("Logs from your program will appear here!");

    
     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
     for stream in listener.incoming() {
         match stream {
             Ok(_stream) => {
                 println!("accepted new connection");

                 stream.write(b"+PONG\r\n").unwrap();
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
