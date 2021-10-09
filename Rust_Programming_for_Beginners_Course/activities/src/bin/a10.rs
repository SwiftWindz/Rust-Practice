// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print


fn var_message(foo: bool){
    // Match statements foo, a bool, to check whether its true or false
    match foo {
        true => println!("It's too big"),
        false => println!("It's too small"),
    }

}


fn main() {
    let num = 111;
    let result;

    if num > 100{
        result = true; 
    } else {
        result = false;
    }

    var_message(result);

}
