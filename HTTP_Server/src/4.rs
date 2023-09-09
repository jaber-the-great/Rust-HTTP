 // Returns an iterator over a string slice
fn main(){
    let request:&str = "Get /search?name=abc HTTP/1.1";
    let mut iter  =request.chars();
    loop {
        let item = iter.next();
        match item{
            Some(c) =>{},
            None => break,
        }
    }
    // The alternative for above code is
    for c in request.chars(){

    }
    // To get index we should use enumerate:
    for (i ,c) in request.chars().enumerate(){

    }
    match get_next_word(request){
        Some((method,request)) =>{},
        None => return Err(ParesError::InvalidRequest),
    }
    // Alternative solution
    // Here we are assigning new value to existing variable which is called variable shadowing
    let (method, request) = get_next_word(request).ok_or(Err((ParesError::InvalidMethod)))?;

    // Three different methods:
    // 1 is match
    let mut query_string = None;
    match path.find('?'){
        Some(i) =>{
            query_string = Some(&path[i+1 ..]);
            path = &path[..i];
        }
        None => {},
    }
    // 2 is if statement
    let q = path.find('?');
    if q.is_some(){
        let i = q.unwrap();
        query_string = Some(&path[i+1..]);
        path = &path[..i];
    }
    // 3 is if let
    if let Some(i) = path.find('?'){
        query_string = Some(&path[i+1..]);
        path = &path[..i];
    }








}

