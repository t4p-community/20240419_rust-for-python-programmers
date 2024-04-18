pub struct Color {
    name: String,
    hex_code: String,
}

impl Color {
    pub fn new(name: &str, hex_code: &str) -> Color {
        Color {
            name: name.to_string(),
            hex_code: hex_code.to_string(),
        }
    }

    pub fn print(&self) {
        println!("Color: {} (#{})", self.name, self.hex_code);
    }
}
