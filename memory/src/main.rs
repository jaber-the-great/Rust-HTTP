use std::io;
fn main() {

    // used for testing memory and stack
    // let a = 2;
    // let result = stack_only(a);
    // dbg!(result);

    // used for testing ownership:
    println!("Enter your weight:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // Passes the reference to string
    borrow_string(&input);
    // Gives the ownership of input string
    // own_string(input);

    let weight:f32 = input.trim().parse().unwrap();
    dbg!(weight);

    let mars_weight = calculate_weight(weight);
    println!("Weight on Mars: {}", mars_weight);

}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d = 5;
    let e = Box::new(7);
    return d + *e;
}

fn borrow_string(s : &String){
    println!("Borrow string: {}",s)
}

fn own_string(s : String){
    println!("Own string: {}",s)
}

fn calculate_weight(weight:f32)->f32{
    (weight/9.81)*3.711
}