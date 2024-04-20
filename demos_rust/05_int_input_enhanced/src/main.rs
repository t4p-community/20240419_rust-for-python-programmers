use std::io::{stdin, stdout, Write};

// parameter prompt is a string slice
// a string slice is a reference to a string
// what is the difference between a String and a &str?
// a String is a heap-allocated string
// a &str is a reference to a string
// String is mutable, &str is immutable
fn num_input(prompt: &str) -> f64 {
    // loop is a keyword that creates an infinite loop
    loop {
        let mut input = String::new();

        print!("{}", prompt);

        stdout().flush().expect("Failed to flush");

        stdin().read_line(&mut input).expect("Failed to read line");

        // ::<i32> is a turbofish
        // it tells the compiler to parse the string into an i32
        // to parse a f64, use ::<f64>
        // match input.trim().parse::<i32>() {
        match input.trim().parse() {
            // Ok is an enum variant that indicates success
            // num is the value that was parsed
            // break is a keyword that exits the loop
            // and returns the value num
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                // continue is a keyword that skips the rest of the loop body
                // and starts the next iteration
                continue;
            }
        }
    }
}

fn main() {
    // the "" syntax creates a string slice
    let num1 = num_input("Enter the first number:");
    let num2 = num_input("Enter the second number:");
    let result = num1 + num2;

    println!("The sum of {} and {} is {}", num1, num2, result);
}
