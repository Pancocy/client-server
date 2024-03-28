pub mod server;
pub mod router;
pub mod handler;

use server::Server;

pub fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}