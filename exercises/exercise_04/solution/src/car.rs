use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Car {
    pub id: i32,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub color: String,
    pub price: f64,
}

impl Car {
    pub fn new(id: i32, make: &str, model: &str, year: i32, color: &str, price: f64) -> Car {
        Car {
            id,
            make: make.to_string(),
            model: model.to_string(),
            year,
            color: color.to_string(),
            price,
        }
    }

    pub fn print(&self) {
        println!(
            "Car: {} {} {} {} {} ${:.2}",
            self.id, self.make, self.model, self.year, self.color, self.price
        );
    }
}
