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

enum Color {
    Red,
    Blue,
    Yellow,
}

fn print_color(color: Color){
    match color {
        Color::Red => println!("The color is red"),
        Color::Blue => println!("The color is blue"),
        Color::Yellow => println!("The color is yellow"),
    }
}

fn main() {
    print_color(Color::Blue);
    print_color(Color::Yellow);
    print_color(Color::Red);
}
