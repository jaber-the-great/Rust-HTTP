match stream.read(&mut buffer){
    Ok(_) => {
        println!("Received a request {}", String::from_utf8_lossy(&buffer));
        match Request::try_from(&buffer as &[u8]){
            Ok(Request) =>{
                dbg!(Request);
                //
                // write!(stream, "HTTP1/1 404 Not Found\r\n\r\n");
                // Next line creates a response like the previous line
                // let response = Response::new(StatusCode::NotFound, None);
                let response = Response::new(StatusCode::Ok,
                     Some("<h1>It worked YaY!!!</h1>".to_string()));
                // write!(stream, "{}",response);
                response.send(&mut stream);
            }
            Err(e) =>{
                println!("Failed to parse the request {}",e);
                Response::new(StatusCode::BadRequest, None).send(&mut stream);
            }
        }
    }
    Err(e) => println!("Failed to read with {}", e),
}



pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code : StatusCode , body:Option<String>) -> Self {
        Response { status_code, body }
    }
    // The reason we are using this method instead of the method on the server.rs is that, the other method
    // Creates a new string(when we want to send the response to the stream)in the heap which may take
    // a lot of space depending on the load of webpage. 
    // So, we move the write function here
    pub fn send(&self, stream:&mut TcpStream) -> IoResult<()>{
        let body = match &self.body{
            Some(b) => b,
            None => "",
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}",
        self.status_code, 
        self.status_code.reason_phrase(),
        body)
    }
    
}

// The following is required if we are using the method of calling the write on tcp stream in server.rs
// impl Display for Response {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         let body = match &self.body{
//             Some(b) => b,
//             None => "",
//         };
//         write!(f, "HTTP/1.1 {} {}\r\n\r\n{}",
//         self.status_code, 
//         self.status_code.reason_phrase(),
//         body)
//     }
    
// }