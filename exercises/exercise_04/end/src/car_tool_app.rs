use crate::car::Car;
use std::{
    io::{stdin, stdout, Write},
    str::FromStr,
};

pub struct CarToolApp {
    pub cars: Vec<Car>,
}

impl CarToolApp {
    pub fn new() -> CarToolApp {
        CarToolApp { cars: Vec::new() }
    }

    fn console_input<T: FromStr>(prompt: &str) -> Result<T, T::Err> {
        print!("{}", prompt);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<T>()
    }

    pub fn add_car(&mut self) {
        let make: String = Self::console_input("Enter car make: ").unwrap();
        let model: String = Self::console_input("Enter car model: ").unwrap();
        let year: i32 = Self::console_input("Enter car year: ").unwrap();
        let color: String = Self::console_input("Enter car color: ").unwrap();
        let price: f64 = Self::console_input("Enter car price: ").unwrap();
        let car = Car::new(&make, &model, year, &color, price);
        self.cars.push(car);
    }

    pub fn show_cars(&self) {
        for car in self.cars.as_slice() {
            car.print();
        }
        let num_of_cars = self.cars.len();
        let total_price_of_all_cars: f64 = self.cars.iter().map(|car| car.price).sum();
        // print num and total
        println!(
            "Cars Count: {}, Total Inventory Value: ${:.2}",
            num_of_cars, total_price_of_all_cars
        );
    }
}
