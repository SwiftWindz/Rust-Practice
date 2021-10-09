// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color{
    // Possible colors of a box
    Red,
    Brown,
    Yellow,
}

struct Box{
    //Characteristics of a box
    length: i32,
    height: i32,
    weight: i32,
    color: Color,
}

impl Box{

    fn create_new_box_red() -> Self {
        //Creates a box with arbitrary values
        Self{
            length: 10,
            height: 5,
            weight: 43,
            color: Color::Red,
        }
    }

    fn create_new_box_yellow() -> Self {
        //Creates a box with arbitrary values
        Self{
            length: 15,
            height: 20,
            weight: 75,
            color: Color::Yellow,
        }
    }

    fn create_new_box_brown() -> Self {
        //Creates a box with arbitrary values
        Self{
            length: 8,
            height: 2,
            weight: 23,
            color: Color::Brown,
        }
    }
    
    fn print_length(&self){
        //Prints a box's length
        println!("Box's length is {:?}", self.length)
    }
    
    fn print_height(&self){
        //Prints a box's height
        println!("Box's height is {:?}", self.height)
    }
    
    fn print_weight(&self){
        //Prints a box's weight
        println!("Box's weight is {:?}", self.weight)
    }

    fn print_color(&self){
        //Prints a box's color
        match self.color{
            Color::Red => println!("Box's color is Red"),
            Color::Brown => println!("Box's color is Brown"),
            Color::Yellow => println!("Box's color is Yellow"),
            _ => println!("Color is unknown"),
        }
    }

}

fn main() {
    let new_box_red = Box::create_new_box_red();
    new_box_red.print_color();
    new_box_red.print_height();
    new_box_red.print_length();
    new_box_red.print_weight();
    println!("");
    let new_box_brown = Box::create_new_box_brown();
    new_box_brown.print_color();
    new_box_brown.print_height();
    new_box_brown.print_length();
    new_box_brown.print_weight();
    println!("");
    let new_box_yellow = Box::create_new_box_yellow();
    new_box_yellow.print_color();
    new_box_yellow.print_height();
    new_box_yellow.print_length();
    new_box_yellow.print_weight();

}
