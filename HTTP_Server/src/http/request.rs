use crate::http::{request, method};
use super::method::{Methods, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult , Formatter , Debug, Display};
use std::str::Utf8Error;
use std::str::from_utf8;
use super::QueryString;


// Defining lifetime here with 'buf
// We need to use this derive to be able to use debugging tool
// the term "derive" refers to a mechanism that allows you to automatically
// generate implementations of certain traits for custom data types
#[derive(Debug)]
pub struct Request<'buf>{
    path: &'buf str, 
    //Before using query string module, was using this line
    // query_string: Option<&'buf str>,
    query_string : Option<QueryString<'buf>>,
    method: Methods,
}

// Writing getter. The convention is to name the getter after the field without prefixing 
// it with the get word
impl<'buf> Request <'buf>{
    pub fn path(&self) -> &str{
        &self.path

    }
    pub fn method(&self) -> &Methods{
        &self.method
    }
    pub fn query_string(&self) -> Option<&QueryString>{
        // as_ref converts from &Option(T) to Option(&T)
        self.query_string.as_ref()
    }
}
// Declaring the lifetime
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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
            // Before using the query string method
            // query_string = Some(&path[i+1..]);
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        Ok(Self { path, query_string, method })       
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