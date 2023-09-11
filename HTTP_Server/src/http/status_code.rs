use std::fmt::{Display, Formatter, Result as FmtResult};
#[derive(Copy, Clone, Debug)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode{
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}
 

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // The * dereferences the pointer
        write!(f, "{}", *self as u16)
    }
}

// If it says does not implement the 'copy trait', it is because the value lives on stack 
// and heap. Types that solely lives in stack can be trivially copied just by 
// copying its bytes. But types like string (the meta data in stack and pointer which 
// points to the heap where the actual data is), if we just copy the bytes, it would only
// copy the metadata --> the string can not implement copy trait. For complex types like
// string, we use clone which provides functionality of creating the deep copy of a value.
// To do that, we use derive(copy) but first we need to derive clone 