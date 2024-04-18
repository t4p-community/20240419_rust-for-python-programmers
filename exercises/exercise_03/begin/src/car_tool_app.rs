use crate::car::Car;
use std::io::{stdin, stdout, Write};

pub struct CarToolApp {
    pub cars: Vec<Car>,
}

impl CarToolApp {
    pub fn new() -> CarToolApp {
        CarToolApp { cars: Vec::new() }
    }

    fn str_input(prompt: &str) -> String {
        let mut input = String::new();
        print!("{}", prompt);
        stdout().flush().expect("Unable to flush stdout");
        stdin().read_line(&mut input).expect("Unable to read input");
        input.trim().to_string()
    }

    fn i32_input(prompt: &str) -> i32 {
        let mut input = String::new();
        print!("{}", prompt);
        stdout().flush().expect("Unable to flush stdout");
        stdin().read_line(&mut input).expect("Unable to read input");
        input
            .trim()
            .parse::<i32>()
            .expect("Unable to parse i32 input")
    }

    fn f64_input(prompt: &str) -> f64 {
        let mut input = String::new();
        print!("{}", prompt);
        stdout().flush().expect("Unable to flush stdout");
        stdin().read_line(&mut input).expect("Unable to read input");
        input
            .trim()
            .parse::<f64>()
            .expect("Unable to parse i32 input")
    }

    pub fn add_car(&mut self) {
        let make = Self::str_input("Enter car make: ");
        let model = Self::str_input("Enter car model: ");
        let year = Self::i32_input("Enter car year: ");
        let color = Self::str_input("Enter car color: ");
        let price = Self::f64_input("Enter car price: ");
        let car = Car::new(&make, &model, year, &color, price);
        self.cars.push(car);
    }

    pub fn show_cars(&self) {
        for car in self.cars.as_slice() {
            car.print();
        }
    }
}
