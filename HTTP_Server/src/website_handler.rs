
use crate::http::request;
use std::fmt::format;
use std::fs;
use super::server::Handler;
use super::http::{Request, Response,StatusCode, Methods};
use std::path::Path;
// pub struct WebsiteHandler;
// Since we want to feed the html from outside, we need to change WebsiteHandler struct
// So, instead of the above line, we create a new struct and impl it
pub struct WebsiteHandler {
    public_path: String
}
impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }
    
    fn read_file(&self, file_path:&str) -> Option<String>{
        // Change slashes based on OS type 
        let path = format!("{}\\{}", self.public_path, file_path);
        
        // Prevent the directory traversal attack by using the real path
        match fs::canonicalize(path){
            Ok(path) => {
                
                // Windows have some issues with the canonicalize(adds four dummy char to the beginning)
                // The following code removes that. It is not needed for linux.
                let my_str = path.into_os_string().into_string().unwrap();
                let correct_path = &my_str[4..];


                // Check .. is not used at the beginning
                if correct_path.starts_with(&self.public_path){
                    println!("{}",correct_path);
                    fs::read_to_string(correct_path).ok()
                }
                else{
                    
                    println!("Directory Traversal Attack Attempted: {}", self.public_path);

                    None
                }
            }
            Err(_) => None,
        }

    }
}
impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        // Response::new(StatusCode::Ok, Some("<h1>Test</h1>".to_string()))
        match request.method(){
            Methods::GET => match request.path(){
                "/" => Response::new(StatusCode::Ok ,self.read_file("index.html")),
                "/jaber" =>Response::new(StatusCode::Ok ,Some("<h1>Jaber The Great!</h1>".to_string()) ),
                "/bio" => Response::new(StatusCode::Ok, self.read_file("bio.html")),
                // _ => Response::new(StatusCode::NotFound, None),
                // This method allows to use the css file but it would result in directory traversal vulnerability
                // We can use .. to go one directory up and read cargo.toml file etc. One solution is checking
                // .. in the path. The other way is convert the path to the canonical version
                path => match self.read_file(path){
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
          _ => Response::new(StatusCode::NotFound, None),
        }
        
    }
    
}