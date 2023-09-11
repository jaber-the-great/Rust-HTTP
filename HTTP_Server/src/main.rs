#![allow(warnings)]
use server::Server;
use website_handler::WebsiteHandler;
// Should make some changes in mod.rs to make the next 
// use work --> pub use request::Request ;

// use http::Methods;
// use http::Request;
mod server;
// It does not work cause http is not file but folder,
// We should add mod.rs to http folder
mod http;
mod website_handler;
use std::env;

fn main() {
    // let server = server::Server::new("127.0.0.1:8080".to_string());
    let server1 = Server::new("127.0.0.1:8080".to_string());
    // server1.run();
    // server1.run(WebsiteHandler);
    // It would read the environment variable from system
    // THe next line reads the env variable at compile time (our default has the same
    // path as cargo.toml). Change slashes for other OS types
    let default_path = format!("{}\\public" , env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("The public path is {}", public_path);
    // let public_path = env::var("PUBLIC_PATH").unwrap();
    server1.run(WebsiteHandler::new(public_path));
    
}

