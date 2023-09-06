use crate::http::request;
use super::method::Methods;
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
    type Error = ParesError;
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let result = std::str::from_utf8(buf)?;
        unimplemented!();       
    }
}

pub enum ParesError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod, 
}
impl ParesError {
    fn message(&self) -> &str {
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }  
}


impl Error for ParesError{
    
}

impl Display for ParesError{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
}
impl Debug for ParesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
    
}
impl From<Utf8Error> for ParesError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}