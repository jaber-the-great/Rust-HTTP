use serv::Server;
use http::request::Request;
fn main() {

    // let get = Methods::GET;
    // let delete = Methods::DELETE;
    // let post: Methods = Methods::POST;
    // let put: Methods = Methods::PUT;


    // let server = serv::Server::new("127.0.0.1:8080".to_string());
    let server = Server::new("127.0.0.1:8080".to_string());
    // server.run();
    
}

mod serv{
    pub struct Server {
        addr : String, 
    }

    impl Server{
        pub fn new(address : String) -> Self{
            Server { addr: address }
        }

        fn run(self) {
            println!("Listening to {}", self.addr);       
        }
    }
}


mod http {
    pub mod request{
        pub struct Request{
            path:String, 
            query_string: Option<String>,
            method: super::Methods,
        }
    }
    pub enum Methods {
        GET, 
        POST,
        PUT,
        DELETE,
        HEAD,
        OPTIONS,
        PATCH,
        CONNECT,
        TRACE
    }
}