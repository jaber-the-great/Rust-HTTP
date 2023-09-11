use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http::request::ParseError;
// Using the keyword trait means the route of entire trait
use crate::http::{Request, response, request};
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::http::{Response, StatusCode};

pub trait Handler{
    fn handle_request(&mut self, request: &Request) -> Response;


    // Directly providing implementation for this function as default implementation
    fn handle_bad_request(&mut self, e:&ParseError) -> Response{
        println!("Failed to parse request: {} ",e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr : String, 
}

impl Server{
    // Returns an instance of server or self 
    pub fn new(address : String) -> Self{
        Server { addr: address }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening to {}", self.addr); 

        let listener = TcpListener::bind(&self.addr).unwrap(); 
        loop{
            match listener.accept(){
                Ok((mut stream , _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer as &[u8]){
                                Ok(Request) =>{
                                    handler.handle_request(& Request)
                                    // instead of sending below static response, we use the handler
                                    // dbg!(Request);
                                    // Response::new(StatusCode::Ok,Some("<h1>It worked YaY!!!</h1>".to_string()))
                                
                                }
                                Err(e) =>{
                                    handler.handle_bad_request(&e)
                                    //Instead of the following lines, we use handler to handle the errors
                                    // println!("Failed to parse the request {}",e);
                                    // Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                print!("Failed to send the response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read with {}", e),
                    }
                }
                // It should be , instead of ; if we use one line 
                Err(e) => println!("We have an error: {}",e),

            }

            let res = listener.accept();
            if res.is_err(){
                continue;
            }
            let (stream, addr) = res.unwrap();
        }

        

    }
}