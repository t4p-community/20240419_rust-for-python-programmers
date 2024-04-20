mod color;

use color::Color;

fn main() {
    // Vec is a growable array
    // mut makes the variable mutable
    let mut colors = Vec::new();

    let color1 = Color::new("red", "ff0000");
    // push adds an element to the Vec
    colors.push(color1);

    let color2 = Color::new("green", "00ff00");
    colors.push(color2);

    let color3 = Color::new("blue", "0000ff");
    // impl { fn push(&mut self, value: T) }
    colors.push(color3);


    // for iterates over the colors Vec
    for color in colors {
        color.print();
    }
}
