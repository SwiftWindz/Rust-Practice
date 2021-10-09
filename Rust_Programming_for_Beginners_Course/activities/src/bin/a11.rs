// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem{
    //struct for a grocery item
    quantity: i32,
    id_number: i32,
    
}

fn display_quantity(grocery_item: &GroceryItem){
    //borrows a grocery item var and displays the quantity
    println!("Quantity = {:?}", grocery_item.quantity)
}

fn display_id_number(grocery_item: &GroceryItem){
    //borrows a grocery item var and displays the id_number
    println!("ID Number = {:?}", grocery_item.id_number)
}

fn main() {
    let item = GroceryItem{
        quantity: 5,
        id_number: 32,
    };

    //Uses '&' which borrows a piece of data and returns it
    display_id_number(&item);
    display_quantity(&item)
}
