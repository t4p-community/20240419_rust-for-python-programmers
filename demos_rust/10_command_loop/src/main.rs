use std::io::{stdin, stdout, Write};

fn get_command(prompt: &str) -> String {
    let mut command = String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut command).unwrap();

    // observe that there is no semicolon - implied return statement
    // it will create a new string, and move it as part of the return
    // caller takes ownership of the string
    command.trim().to_string()
}

fn main() {
    // loop is an infinite loop
    loop {
        let command = get_command("Enter command: ");

        // match is like a switch statement
        // convert the String command to a string slice
        match command.as_str() {
            // in Rust string literals are &str (slices)
            "exit" => {
                println!("Exiting...");
                // break exits the loop
                break;
            }
            // _ is a catch-all pattern
            _ => {
                println!("Command: {}", command);
            }
        }
    }
}
