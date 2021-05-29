// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Cola,
    Grape,
    Orange,
}

struct Drink {
    fluid_ounces: f64,
    flavor: Flavor
}

fn print_drink(drink: Drink){
    let flavor;
    let ounces = drink.fluid_ounces;

    match drink.flavor {
        Flavor::Cola => flavor = "Cola", 
        Flavor::Grape => flavor = "Grape", 
        Flavor::Orange => flavor = "Orange", 
    }

    println!("Flavor: {:?}\nOunces: {:?}\n", flavor, ounces);
}

fn main() {

    let grape_drink = Drink{
        fluid_ounces: 20.0,
        flavor: Flavor::Grape
    };
    let cola_drink = Drink{
        fluid_ounces: 15.7,
        flavor: Flavor::Cola
    };
    let orange_drink = Drink{
        fluid_ounces: 11.5,
        flavor: Flavor::Orange
    };

    print_drink(grape_drink);
    print_drink(cola_drink);
    print_drink(orange_drink);

}
