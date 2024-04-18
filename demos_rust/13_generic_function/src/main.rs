use std::io::{stdin, stdout, Write};
use std::str::FromStr;

// Generic functions enable code reuse for different types that implement the same behavior
// Traits like FromStr are used to constrain the types that can be used with the function

// <T> is a generic type parameter
// T: FromStr is a trait bound - it requires that T implements the FromStr trait
// Because T: FromStr, we can call the parse method on any type T
// Result<T, T::Err> is the return type of the function - if parsing fails, we return an error
// T::Err is an associated type of the FromStr trait
// Rust Generics are monomorphized - the compiler generates a new version of the function for each type
fn console_input<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
    print!("{}", prompt);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse::<T>()
}

fn main() {
    // The generic type param for the function is inferred from the type of the variable: i32
    let value: i32 = console_input("Enter an integer: ")
        .expect("Invalid input. Please enter a value of type i32.");
    println!("You entered integer: {}", value);

    // The generic type param for the function is inferred from the type of the variable: f64
    let value: f64 =
        console_input("Enter a float: ").expect("Invalid input. Please enter a value of type f64.");
    println!("You entered float: {}", value);

    // The generic type param for the function is inferred from the type of the variable: String
    let value: String = console_input("Enter a string: ")
        .expect("Invalid input. Please enter a value of type String.");
    println!("You entered string: {}", value);
}
