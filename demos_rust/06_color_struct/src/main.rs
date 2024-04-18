// Rust does not have classes like Python, but it has structs
// structs are used to create custom data types

// a struct is a custom data type that lets you name and package together multiple related values
// structs are passed by value by default
struct Color {
    name: String,
    hex_code: String,
}

// ownership of the color struct is moved to the print_color function
fn print_color(color: Color) {
    println!("Color: {} (#{})", color.name, color.hex_code);
}

// ownership of the color struct is borrowed by the print_color function
// fn print_color(color: &Color) {
//     println!("Color: {} (#{})", color.name, color.hex_code);
// }

fn main() {
    let color1 = Color {
        name: String::from("red"),
        hex_code: String::from("ff0000"),
    };

    print_color(color1);

    // print_color(&color1);
    // print_color(&color1);
}
