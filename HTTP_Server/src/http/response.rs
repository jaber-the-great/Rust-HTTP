use super::StatusCode;
use std::net::TcpStream;
use std::io::{Write,Result as IoResult};
use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code : StatusCode , body:Option<String>) -> Self {
        Response { status_code, body }
    }
 
    // Making the function more general by using a general stream rather than 
    // TcpStrem. We need to use "dyn" to make it dynamic dispatch 
    // We can also use static dispatch, to do so replace "dyn" with "impl"
    // pub fn send(&self, stream:&mut dyn  Write ) -> IoResult<()>
    pub fn send(&self, stream:&mut impl  Write ) -> IoResult<()>{
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
