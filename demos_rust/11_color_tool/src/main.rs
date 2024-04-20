mod color;

use color::Color;
use std::io::{stdin, stdout, Write};

fn get_input(prompt: &str) -> String {
    let mut command = String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut command).unwrap();

    command.trim().to_string()
}

// &mut passes a mutable reference
// a mutable reference allows the function to push colors to the array
fn add_color(colors: &mut Vec<Color>) {
    let name = get_input("Enter color name: ");
    let hex_code = get_input("Enter hex code: ");
    let color = Color::new(&name, &hex_code);
    colors.push(color);
}

// & passes an immutable reference
// an immutable reference allows the function to read the colors from the array
// the array cannot be mutated
fn show_colors(colors: &Vec<Color>) {
    for color in colors {
        color.print();
    }
}

fn main() {
    // mut declares a mutable variable
    // let mut colors: Vec<Color> = Vec::new();
    let mut colors = Vec::new();

    loop {
        let command = get_input("Enter command: ");
        match command.as_str() {
            "add" => {
                // &mut passes a mutable reference
                add_color(&mut colors);
            }
            "show" => {
                // & passes an immutable reference
                show_colors(&colors);
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
