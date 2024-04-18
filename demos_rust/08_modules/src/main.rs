// mod declares the mod color module
mod color;

// use makes the module available to the current scope
use color::Color;

fn main() {
    let color1 = Color::new("red", "ff0000");
    color1.print();
}
