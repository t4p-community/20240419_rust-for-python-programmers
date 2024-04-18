use std::io::{stdin, stdout, Write};

// fn is a keyword that declares a function
// i32 is the return type of the function
fn num_input() -> i32 {
    let mut input = String::new();

    print!("Please enter a number: ");

    // expect() is a method that takes a string and returns the value of the Ok variant
    // If the Result is an Err, expect() will panic and display the string
    // expect is an alternative to match that is more concise
    stdout().flush().expect("Failed to flush");

    stdin().read_line(&mut input).expect("Failed to read line");

    // parse() is a method that converts a string to a number
    // if parse is successful, it returns a Result with the Ok variant
    // the Ok variant contains the number
    return input.trim().parse().expect("The input is not a number");
}

fn main() {
    // call num_input() and assign the return value to num1
    // the type of num1 is i32, inferred by the compiler from the return type of num_input()
    let num1 = num_input();
    let num2 = num_input();
    let result = num1 + num2;

    println!("The sum of {} and {} is {}", num1, num2, result);
}
