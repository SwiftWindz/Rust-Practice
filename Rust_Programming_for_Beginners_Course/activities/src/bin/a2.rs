// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

//Adds two given numbers together
fn addition(a: i32, b: i32) -> i32{
    return a + b;
}

//Prints a 32bit int
fn num_printer(a: i32){
    println!("{:?}", a);
}

fn main() {
    num_printer(addition(10, 10));
}
