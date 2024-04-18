use crate::car::Car;

use std::fs::{read_to_string, write};
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

        let max_id = self.cars.iter().map(|car| car.id).max().unwrap_or(0);

        let car = Car::new(max_id + 1, &make, &model, year, &color, price);
        self.cars.push(car);
    }

    pub fn remove_car(&mut self) {
        let id: i32 = Self::console_input("Enter car id to remove: ").unwrap();
        let index = self.cars.iter().position(|car| car.id == id);
        match index {
            Some(i) => {
                self.cars.remove(i);
                println!("Car with id {} removed successfully", id);
            }
            None => println!("Car with id {} not found", id),
        }
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

    pub fn save_cars(&self) {
        let file_name: String =
            Self::console_input("Enter the name of the file to save the cars to: ").unwrap();
        let cars_serialized = serde_json::to_string(&self.cars).unwrap();
        write(file_name.as_str(), cars_serialized).unwrap();
    }

    pub fn load_cars(&mut self) {
        let file_name: String =
            Self::console_input("Enter the name of the file to load the cars from: ").unwrap();
        let cars_serialized = read_to_string(file_name.as_str()).unwrap();
        let cars_deserialized: Vec<Car> = serde_json::from_str(&cars_serialized).unwrap();
        self.cars = cars_deserialized;
    }
}
