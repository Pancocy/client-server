use std::net::TcpListener;

use std::io::{Read,Write};

fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();

    println!("Running at port 3000......");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        println!("Connection Established!");

        let mut buffer = [0;1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }


}
