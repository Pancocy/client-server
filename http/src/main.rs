pub mod server;
pub mod router;
pub mod handler;

use server::Server;

fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}