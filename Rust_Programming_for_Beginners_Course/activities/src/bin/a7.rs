// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Blue,
    Yellow,
}

fn print_color(color: Colors){
    match color {
        Colors::Red => println!("The color is red"),
        Colors::Blue => println!("The color is blue"),
        Colors::Yellow => println!("The color is yellow"),
    }
}

fn main() {
    print_color(Colors::Blue);
    print_color(Colors::Yellow);
    print_color(Colors::Red);
}
