// The Display trait is used to format the output of a type in a more human-readable way.
use std::fmt::Display;

struct CartItem {
    name: String,
    price: f32,
    quantity: i32,
}

impl CartItem {
    fn new(name: &str, price: f32, quantity: i32) -> Self {
        Self {
            name: name.to_string(),
            price,
            quantity,
        }
    }
}

// Implement the Display trait for the CartItem struct
// Traits are similar to interfaces in other languages
// In Python an interface can be implement using an Abstract Class or Protocol
impl Display for CartItem {
    // Implement the fmt method for the Display trait
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Format the output of the CartItem struct and write it to the Formatter
        write!(
            f,
            "Item: {}, Quantity: {}, Price: ${:.2}",
            self.name, self.quantity, self.price
        )
    }
}

fn main() {
    // vec! is a macro that creates a Vec<T> from a list of items
    let cart = vec![
        CartItem::new("Apple", 1.99, 2),
        CartItem::new("Banana", 0.99, 3),
        CartItem::new("Orange", 2.99, 1),
    ];

    for item in &cart {
        // Because the CartItem struct implements the Display trait, we can use the println! macro
        // to print the CartItem struct
        // This is similar to having __str__ method on a CartItem class in Python
        println!("{}", item);
    }

    let total: f32 = cart
        // iter method returns an iterator over the items in the cart vector
        // Python does something similar with the __iter__ method
        .iter()
        // map method applies a function to each item in the iterator
        // Python supports a map function that works similarly
        // this map will calculate the subtotal for each item in the cart
        // the "as f32" is used to convert the quantity to a float
        .map(|item| item.price * item.quantity as f32)
        // sums the subtotals
        .sum();
    println!("Total: ${:.2}", total);
}
