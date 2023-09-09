use std::net::TcpListener;       
// bind, instead or returning a tcp listener, returns result,
// which is at the core of error handling in rust
// recoverable and unrecoverable errors, most langs do not distinguish between
// these two and use exception. Rust distinguishes between them but does not have 
// exception. It uses enums to handle errors 
// If we use unwrap,it would return the value of result if the value is OK,
// but if it is an error, it would terminate our program and print error on screen
let listener = TcpListener::bind(&self.addr);      
       
while true {
}
// Loop is like while true and  
loop {

    // continue;
    // break;
}
// also we can label the loops: ' + label name
'outer: loop{
    'inner: loop{
        break 'outer;
        continue 'inner;
    }
}

// Rust uses tuples to compound several values and is immutable, can be
// used to return several things from a function
tup = (5, "here", listener);

// To help work better with enums, rust has match for better management and easier coding.
// instead of having if else to catch all errors before unwrapping, matching can be used
// The code would not compile until we cover all the match cases 
loop{
    match listener.accept(){
        // Ok wraps two values as tuple
        // use _ instead of the values if we don't want to use them , 
        // or use _ for each of the values we don't want 
        Ok(_) => println!("don't want to use the value"),
        Ok((stream, _)) => println!("just use the stream not the address"),       
        Ok((stream , addr)) => {
            println!("OK");
            // for using stream.read(), we should include read cause it is implemented separately
            // stream should become mutable and we should pass buffer
        }
        // It should be , instead of ; if we use one line 
        Err(e) => println!("We have an error: {}",e),
        // for catching all the variants that we have not matched manually 
        _ => {},

    }

    // Match is not only for enums, it can be used as switch case
    let mystr = "Abcd";
    match mystr {
        "Abcd" => println!("Matched!"),
        "a" | "b"=> println!("case a or b"),
        _ => {}

    }

    let res = listener.accept();
    if res.is_err(){
        continue;
    }
    let (stream, addr) = res.unwrap();
}

// We should define the size of array (5 here) otherwise it does not work
fn arr(a: [u8; 5]) {}
// The other solution is passing by reference,the compiler knows how big pointers are  
// It is lik string slice 
fn arr(a: &[u8]) {}
let x = [1,2,3,4,5,6,7];
arr(&x[2..4]);
// create array with same value for each element
let y = [0;1024];

loop{
    match listener.accept(){

        Ok((mut stream , _)) => {
            println!("OK");
            let mut buffer = [0;1024];
            stream.read(&mut buffer);
        }
        // It should be , instead of ; if we use one line 
        Err(e) => println!("We have an error: {}",e),

    }
}