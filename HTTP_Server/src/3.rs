use super::method::Methods;
use std::convert::TryFrom;
use std::convert::TryInto;
pub struct Request{
    path:String, 
    query_string: Option<String>,
    method: Methods,
}


impl Request{
    fn from_byte_array(buf: &[u8]) -> Result<self, String>{}
}


// Traits are similar to interface in other languages
impl TryFrom<&[u8]> for Request {
    // We need to implement a function and define Error alias for this interface 
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let string = String::from("abcd");
        string.encrypt();
        value.encrypt();
        unimplemented!();       
    }
}

// We can define a trait and implement it for different types
trait Encrypt{
    fn encrypt(&self) ->self;
}

impl Encrypt for String {
    fn encrypt(&self) ->self {
        unimplemented!();
    }
}
impl Encrypt for &[u8]{
    fn encrypt(&self) ->self {
        unimplemented!();
    }
}

match listener.accept(){
    Ok((mut stream , _)) => {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer){
            Ok(_) => {
                println!("Received a request {}", String::from_utf8_lossy(&buffer));
                // We need to tell the compiler which type of data we are using
                Request::try_from(&buffer as &[u8]);
                // The other way of that is directly converting that
                Request::try_from(&buffer[..]);
                // Or we can use try_into, but we need to define the return type till 
                // the compiler knows which function to use
                let res: &Result<Request,_> = &buffer[..].try_into();
            }
            Err(e) => println!("Failed to read with {}", e),
        }
    }
}

// Looking at the doc, it can only be used for the functions that implement Debug and Display
impl Error for ParesError{
    
}

impl Display for ParesError{
    // if we do not import, we should use this way
    // fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //     unimplemented!();
    // }

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
}
impl Debug for ParesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}", self.message())
    }
    
}


fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
    match str::from_utf8(buf){
        Ok(request) => {},
        Err(_) => return Err(ParesError::InvalidEncoding),
    }
    // This is another way of error handling 
    match str::from_utf8(buf).or(Err(ParesError::InvalidEncoding)){
        Ok(request)=> {},
        Err(e) => return Err(e),
    }
    // This is the third way, using a special syntx of ?. 
    let request = str::from_utf8(buf).or(Err(ParesError::InvalidEncoding))?;
    // The ? tries to convert 
    // the error type if it does not match the error type the funtion trying to return
    let request = str::from_utf8(buf)?;
    // To use the last one we need to implement "From"
    
    unimplemented!();       
}
use std::str::Utf8Error;
impl From<Utf8Error> for ParesError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}