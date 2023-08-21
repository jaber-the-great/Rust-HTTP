use serv::Server;
use http::request::Request;
fn main() {

    let get = Methods::GET("abcd".to_string());
    let delete = Methods::DELETE(1234);
    let post: Methods = Methods::POST;
    let put: Methods = Methods::PUT;

    let string = String::from("127.0.0.1:8000");
    // String slice: Borrowing an existing string
    let ip = &string[..9];
    // Emojis and special chars are encoded in 4 bytes instead of one 
    let port = &string[10..14]; // give me everything after the 10th byte not the 10th charcter

    // str and string are dirrerent:
    // Normal strings can be expanded and shrinked but str is immutable 
    let string_borrow: &str = &string;
    let string_literal = "1234"; // immutable 


    dbg!(&string); // can not be owned cause already borrowed
    dbg!(ip);
    dbg!(string_borrow);
    dbg!(string); // can be owned cause the borrow is released 
    dbg!(string_literal);
    
    
    

    // let server = serv::Server::new("127.0.0.1:8080".to_string());
    // to_string converts string literal to a string 
    let server = Server::new("127.0.0.1:8080".to_string());
    // server.run();
    
}

mod serv{
    // Similar to class in C++ and Java
    pub struct Server {
        addr : String, 
    }

    impl Server{
        // Returns an instance of server or self 
        pub fn new(address : String) -> Self{
            Server { addr: address }
        }
        // this in java/c++
        // Self takes the ownership of the instance 
        // Can make self a refernce or mutable reference
        // Here run can take the ownership without any issue 
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

// The orig number start from 0 to n
// we can specify a number, here if we put Head =15, options  would be 16 and so on
// We can also have enums which have different types of data like string etc
// When we specify that a varible is of type methods but don't specify the type (it is get or post etc), the 
// complier would assign space as large as the largest enum (cause it does not know which method is going to be used)
pub enum Methods {
    GET(String),
    POST,
    PUT,
    DELETE(u64),
    // HEAD = 15,
    OPTIONS,
    PATCH,
    CONNECT,
    TRACE
}

// Rust does not have non values, instead it uses enum Options:
// T is like C++ -> Generic over some types 
pub enum Options<T> {
    None,
    Some(T),
}
// Using the option for none value 
struct req {
    path: String,
    query_string: Option<String>,
    method: Methods,
}
