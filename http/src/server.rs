use std::io::Read;
use std::net::TcpListener;
use httplib::httprequest::HttpRequest;
use super::router::Router;

pub struct Server<'a>{
    socket_address:&'a str
}

impl <'a> Server <'a>{
    pub fn new(path:&'a str) -> Server<'a>{
        Server{
            socket_address:path
        }
    }
    pub fn run(&self){
        let listenner = TcpListener::bind(self.socket_address).unwrap();
        println!("Running on {}",self.socket_address);
        for stream in listenner.incoming(){
            println!("Established Connection");
            let mut stream = stream.unwrap();
            let mut buffer = [1;1024];
            stream.read(&mut buffer).unwrap();
            let req:HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::route(req,&mut stream)
        }
    }
}