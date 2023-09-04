use std::net::TcpListener;

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

                Ok((stream , addr)) => {
                    println!("OK");
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