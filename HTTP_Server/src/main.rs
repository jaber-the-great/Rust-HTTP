#![allow(warnings)]
use server::Server;

// Should make some changes in mod.rs to make the next 
// use work --> pub use request::Request ;

// use http::Methods;
// use http::Request;
mod server;
// It does not work cause http is not file but folder,
// We should add mod.rs to http folder
mod http;
fn main() {
    // let server = server::Server::new("127.0.0.1:8080".to_string());
    let server1 = Server::new("127.0.0.1:8080".to_string());
    server1.run();
    
}

