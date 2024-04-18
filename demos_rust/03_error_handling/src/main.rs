use std::io::{stdin, stdout, Write};

fn main() {
    let mut name = String::new();

    print!("Please enter your name: ");

    // flush() returns a Result, so we need to handle the error
    // Result is an enum with two variants: Ok and Err
    match stdout().flush() {
        // Err(e) is a pattern that matches any Err value and binds it to e
        Err(e) => println!("Failed to flush: {}", e),
        // Ok(_) is a pattern that matches any Ok value and ignores the value
        Ok(_) => match stdin().read_line(&mut name) {
            Err(e) => println!("Failed to read line: {}", e),
            Ok(_) => {
                let name = name.trim();
                println!("Hello, {}!", name);
            }
        },
    }
}
