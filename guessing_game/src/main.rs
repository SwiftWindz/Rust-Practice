// This is a use statement, works much like an import statement.
// std is the standard library, io is the input/output library 
use std::io;
//From the rand library that we down load in the toml file, import Rng
use rand::Rng;
//From the cmp library in the std library, import Ordering
use std::cmp::Ordering;

//Main dec
fn main() {
    
    //Standard println - prints a string to CLI with a newline char
    println!("Guess the number!");

    //Creates immutable var with a random num between 1-101
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    //Normal printline, but includes formating to display the 'secret_number' var
    println!("The secret number is: {}", secret_number);

    //Standard println - prints a string to CLI with a newline char
    println!("Please input your guess.");

    // Let declares a var
    // mut makes a var mutable - vars are immutable by default in Rust
    // String::new() declares a new str - does not take a str, only declares  
    // the "::" in String::new() shows association between the str type and new func 
    let mut guess = String::new(); 

    //Call to stdin func from io library declared at top of the file
    //stdin returns an instance of "std::io::Stdin" which represents a handle to the standard input for your terminal
    io::stdin()
        //Calls the read_line method on the stdin handle
        //passes in one arg (guess), and writes the input from CLI to that var
        //the "&" indicates that its a reference to the guess var - references are immutable by default, mut allows it to be mutated
        .read_line(&mut guess)
        //results from read_line are enums - either 'Ok' or 'Err'
        //if an 'Err', or error, is returned, the expected method is hit
        .expect("Failed to read line");
    
    //Trim() - gets rid of whitespace
    //u32 seen here is an unsigned, 32-bit integer
    //Parse makes it into a u32
    //expect is error handling again
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    //Normal printline, but includes formating to display the 'guess' var
    //'{}'s are placeholders that show that you wish to display a var that you passed in as an arg - you can do this with many values
    println!("You guessed: {}", guess);

    //cmp method compares two things - here 'guess' is compared to 'secret_number'
    //cmp returns and ordering enum
    //A match statement, matches the return with one of the statements within it (similar to a switch statement), also known as arms
    match guess.cmp(&secret_number){
        //Ordering is an enum - with 'Less', 'Greater', and 'Equal'
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too large"),
        Ordering::Equal => println!("You win"),
    }
}