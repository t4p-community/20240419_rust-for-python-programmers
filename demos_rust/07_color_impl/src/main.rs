// structs combined with impl blocks are used to create custom data types with methods
struct Color {
    name: String,
    hex_code: String,
}

// an impl block is used to define methods on a struct
impl Color {
    // a associated function is a function that is associated with a struct
    // Rust does not have constructors, but you can use associated functions to create instances of a struct
    // associated functions are called using the :: syntax
    // similar to a class method in Python
    fn new(name: &str, hex_code: &str) -> Color {
        Color {
            // to_string() is used to convert a string slice to a String
            name: name.to_string(),
            hex_code: hex_code.to_string(),
        }
    }

    // self is a borrowed reference to the struct instance
    // method functions are called using the . syntax
    // similar to an instance method in Python
    fn print(&self) {
        println!("Color: {} (#{})", self.name, self.hex_code);
    }
}

fn main() {
    let color1 = Color::new("red", "ff0000");
    color1.print();
}
