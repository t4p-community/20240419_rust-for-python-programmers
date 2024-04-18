use crate::color_tool_app::ColorToolApp;
use std::io::{stdin, stdout, Write};

// 'a is a lifetime parameter that is used to ensure that the reference to the ColorToolApp
// instance is valid for the duration of the CommandLoop instance.
pub struct CommandLoop<'a> {
    app: &'a mut ColorToolApp,
}

// 'a is a lifetime parameter that is used to ensure that the reference to the ColorToolApp
// instance is valid for the duration of the CommandLoop instance.
impl<'a> CommandLoop<'a> {
    pub fn new(app: &'a mut ColorToolApp) -> CommandLoop {
        CommandLoop { app }
    }

    fn get_command() -> String {
        let mut command = String::new();

        print!("Enter command: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut command).unwrap();

        command.trim().to_string()
    }

    fn command_exit() {
        println!("Exiting...");
    }

    fn command_unknown(command: &str) {
        println!("Unknown command: {}", command);
    }

    pub fn run(&mut self) {
        loop {
            // Self refers to the CommandLoop struct itself.
            // Self::get_command is like a class method in Python
            let command = Self::get_command();
            match command.as_str() {
                "add" => {
                    self.app.add_color();
                }
                "show" => {
                    self.app.show_colors();
                }
                "exit" => {
                    Self::command_exit();
                    break;
                }
                _ => {
                    Self::command_unknown(command.as_str());
                }
            }
        }
    }
}
