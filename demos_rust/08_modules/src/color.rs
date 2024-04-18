// pub makes the struct public so it can be accessed from outside the module
pub struct Color {
    name: String,
    hex_code: String,
}

impl Color {
    // pub makes the function public so it can be accessed from outside the module
    pub fn new(name: &str, hex_code: &str) -> Color {
        Color {
            name: name.to_string(),
            hex_code: hex_code.to_string(),
        }
    }

    // pub makes the function public so it can be accessed from outside the module
    pub fn print(&self) {
        println!("Color: {} (#{})", self.name, self.hex_code);
    }
}
