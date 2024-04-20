// Rust does not have classes like Python, but it has structs
// structs are used to create custom data types

// a struct is a custom data type that lets you name and package together multiple related values
// structs are passed by value by default
struct Color {
    name: String,
    hex_code: String,
}

// ownership of the color struct is moved to the print_color function
// fn print_color(color: Color) {
//     println!("Color: {} (#{})", color.name, color.hex_code);
// }

// ownership of the color struct is borrowed by the print_color function
fn print_color(color: &mut Color) {
    color.name = String::from("blue");
    println!("Color: {} (#{})", color.name, color.hex_code);
}

fn print_color2(color: &mut Color) {
    println!("Color: {} (#{})", color.name, color.hex_code);
}

fn main() {
    let mut color1 = Color {
        name: String::from("red"),
        hex_code: String::from("ff0000"),
    };

    color1.name = String::from("green");

    // moving the color1 struct ownership to the print_color function
    //print_color(color1);

    print_color(&mut color1);
    print_color2(&mut color1);
}
