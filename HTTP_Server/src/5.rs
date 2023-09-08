// In rust, if our code complies successfully, there is no dangling references
// Rust is the only language explicitly manages lifetime
// Life time session is important
use crate::http::{request, method};
use super::method::{Methods, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult , Formatter , Debug, Display};
use std::str::Utf8Error;
use std::str::from_utf8;

pub struct Request{
    path:String, 
    query_string: Option<String>,
    method: Methods,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        //GET /search?name=abc&sort=1 HTTP/1.1\r\n....rest_of_header
        let request = std::str::from_utf8(buf)?;

        let (method ,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path ,request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol ,_) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }

        let method : Methods = method.parse()?;
        
        let mut query_string = None;

        if let Some(i) = path.find('?'){
            query_string = Some(&path[i+1..]).to_string();
            path = &path[..i];
        }
        
        Ok(Self { path: path.to_string(), query_string, method })       
    }
}

fn get_next_word(request:&str) -> Option<(&str , &str)> {
   for (i,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i + 1..]));
        }
   }
   None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod, 
}
impl ParseError {
    fn message(&self) -> &str {
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }  
}


impl Error for ParseError{
    
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
    
}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}