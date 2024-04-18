use crate::car_tool_app::CarToolApp;
use std::io::{stdin, stdout, Write};

pub struct CommandLoop<'a> {
    app: &'a mut CarToolApp,
}

impl<'a> CommandLoop<'a> {
    pub fn new(app: &'a mut CarToolApp) -> CommandLoop {
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
            let command = Self::get_command();
            match command.as_str() {
                "add" => {
                    self.app.add_car();
                }
                "remove" => {
                    self.app.remove_car();
                }
                "save" => {
                    self.app.save_cars();
                }
                "load" => {
                    self.app.load_cars();
                }
                "show" => {
                    self.app.show_cars();
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
