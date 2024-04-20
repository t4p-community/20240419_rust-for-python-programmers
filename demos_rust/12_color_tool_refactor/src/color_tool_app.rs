use crate::color::Color;
use std::io::{stdin, stdout, Write};

pub struct ColorToolApp {
    pub colors: Vec<Color>,
}

impl ColorToolApp {
    pub fn new() -> Self {
        Self { colors: Vec::new() }
    }

    // the method is now a static method
    // it does not have access to the instance of the struct
    fn console_input(prompt: &str) -> String {
        let mut command = String::new();

        print!("{}", prompt);
        stdout().flush().unwrap();
        stdin().read_line(&mut command).unwrap();

        command.trim().to_string()
    }

    pub fn add_color(&mut self) {
        // Self refers to the ColorToolApp struct itself.
        let name = Self::console_input("Enter color name: ");
        let hex_code = Self::console_input("Enter hex code: ");
        let color = Color::new(&name, &hex_code);
        self.colors.push(color);
    }

    pub fn show_colors(&self) {
        // the as_slice method is used to convert the vector to a slice
        // so that we can iterate over the colors
        // without the as_slice method, we would get an error
        // because the vector would be moved
        // and we would not be able to use it again
        for color in self.colors.as_slice() {
            color.print();
        }
    }
}
