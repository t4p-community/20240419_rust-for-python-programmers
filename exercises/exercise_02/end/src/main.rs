use std::io::{stdin, stdout, Write};

struct Car {
    make: String,
    model: String,
    year: i32,
    color: String,
    price: f64,
}

impl Car {
    fn new(make: &str, model: &str, year: i32, color: &str, price: f64) -> Car {
        Car {
            make: make.to_string(),
            model: model.to_string(),
            year,
            color: color.to_string(),
            price,
        }
    }

    fn print(&self) {
        println!(
            "Car: {} {} {} {} {}",
            self.make, self.model, self.year, self.color, self.price
        );
    }
}

// yes, these functions are repetitive, but they are simple and easy to understand
// for where we are in our Rust journey

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

fn main() {
    let mut count = 0;

    loop {
        let car_make = str_input("Enter car model: ");
        let car_model = str_input("Enter car make: ");
        let car_year = i32_input("Enter car year: ");
        let car_color = str_input("Enter car color: ");
        let car_price = f64_input("Enter car price: ");

        let car = Car::new(&car_make, &car_model, car_year, &car_color, car_price);
        car.print();

        count += 1;
        if count > 2 {
            break;
        }
    }
}
