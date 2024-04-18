pub struct Car {
    make: String,
    model: String,
    year: i32,
    color: String,
    price: f64,
}

impl Car {
    pub fn new(make: &str, model: &str, year: i32, color: &str, price: f64) -> Car {
        Car {
            make: make.to_string(),
            model: model.to_string(),
            year,
            color: color.to_string(),
            price,
        }
    }

    pub fn print(&self) {
        println!(
            "Car: {} {} {} {} {}",
            self.make, self.model, self.year, self.color, self.price
        );
    }
}
