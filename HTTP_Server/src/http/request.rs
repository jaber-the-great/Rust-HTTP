use super::method::Methods;
pub struct Request{
    path:String, 
    query_string: Option<String>,
    method: Methods,
}