use std::io::{stdin, stdout, Write};

fn get_command(prompt: &str) -> String {
    let mut command = String::new();

    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut command).unwrap();

    command.trim().to_string()
}

fn main() {
    // loop is an infinite loop
    loop {
        let command = get_command("Enter command: ");
        // match is like a switch statement
        match command.as_str() {
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
