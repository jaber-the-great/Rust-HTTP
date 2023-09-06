use std::io::Read;
use std::net::TcpListener;
// Using the keyword trait means the route of entire trait
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
pub struct Server {
    addr : String, 
}

impl Server{
    // Returns an instance of server or self 
    pub fn new(address : String) -> Self{
        Server { addr: address }
    }

    pub fn run(self) {
        println!("Listening to {}", self.addr); 

        let listener = TcpListener::bind(&self.addr).unwrap(); 
        loop{
            match listener.accept(){
                Ok((mut stream , _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer as &[u8]){
                                Ok(Request) =>{},
                                Err(e) => print!("Failed to parse the request {}",e),
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