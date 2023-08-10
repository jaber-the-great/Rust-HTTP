use std::io;
fn main() {
    println!("Hello, world!");
    let x: i32 =2; 
    let y = x; 
    let res: i32 = stack_only(x);


    // Complex types like string are stored in the heap but simple types like string live in the stack
    let mut input = String::new();
    

    // The following line raises an error due to the fact each value can have only one owner
    // let mut copyInput = input ;
    // Gives error since passing the ownership 
    // temp(input);
    // temp1(&input);
    // Passing refrence instead of ownership
    // We can only have a sinlge mutable borrow or infinite immutable borrows 
    // temp2(&mut input);

    println!("Please enter your weight:");
    // Unwrap: if the result is error, it would terminate our program but if it is seccuessul, it would yeild to the result 
    io::stdin().read_line(&mut input).unwrap();

    // Parse the string to float 
    let weight: f32 = input.trim().parse().unwrap();



    println!("Weight on marc is {}, string: {}", calculate(weight), "a bit less heavy");
    let mut newWeight: f32 = calculate(57.5);
    newWeight = newWeight * 1000.0;

}

fn calculate(weight: f32)-> f32 {
    
    //implecetly return by not putting ; at the end of last expression 
    //60.2
    (weight / 9.81) * 3.711
}

fn stack_only(y : i32) -> i32 {
    let z = 3;
    return y + z + stack_and_heap();
}

fn stack_and_heap() -> i32{
    let w =5;
    let p = Box::new(7);
    return w + *p;
}

fn temp(str: String){
    // Do nothing
}

fn temp1(str: &String){
    // Passing the reference instead of ownership 
    // Raises error cause it is an immutable reference
    // str.push_str("foo");
}

fn temp2(str: &mut String){
    str.push_str("foo");
    println!("String: {}", str);
}