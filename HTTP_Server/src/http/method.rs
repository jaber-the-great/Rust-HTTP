use std::str::FromStr;
pub enum Methods {
    GET, 
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    CONNECT,
    TRACE,
}

impl FromStr for Methods {
    type Err = MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "GET"=> Ok(Self::GET), 
            "POST"=> Ok(Self::POST),
            "PUT"=> Ok(Self::PUT),
            "DELETE"=> Ok(Self::DELETE),
            "HEAD"=> Ok(Self::HEAD),
            "OPTIONS"=> Ok(Self::OPTIONS),
            "PATCH"=> Ok(Self::PATCH),
            "CONNECT"=> Ok(Self::CONNECT),
            "TRACE"=> Ok(Self::TRACE),
            _ => Err(MethodError),
        }
        
    }
    
}

pub struct MethodError;