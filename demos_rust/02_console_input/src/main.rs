// use brings a module into scope
// std is the standard library
// io is the input/output module
// stdin is the standard input
// stdout is the standard output
// Write is a trait for writing bytes
// traits are similar to interfaces in other languages
// stdout implements the Write trait
use std::io::{stdin, stdout, Write};

fn main() {
    // let declares a variable
    // mut makes the variable mutable
    // String::new() creates a new empty string
    let mut name = String::new();

    // print! is like println! but without a newline
    print!("Please enter your name: ");

    // flush() is needed to ensure the prompt is displayed before the user types
    // unwrap() is used to handle any errors
    // if an error occurs, the program will panic and exit
    stdout().flush().unwrap();

    // stdin() returns a handle to the standard input
    // read_line() reads a line from the standard input into a string
    // the string is a mutable reference so read_line can modify it
    // unwrap() is used to handle any errors
    // if an error occurs, the program will panic and exit
    stdin().read_line(&mut name).unwrap();

    // let redefines the variable name
    // trim() removes leading and trailing whitespace
    let name = name.trim();

    // println! is used to print a formatted string
    // {} is a placeholder for the variable name
    println!("Hello, {}!", name);
}
