# Exercise 4 - Car Tool with File I/O

## Instructions

To complete the lab exercise, review the in-class demonstrations. This exercise combines code from the various demonstrations. Also, you may used generative AI tools, searching, and official documentation to complete the exercise.

### Code Folders - PLEASE READ

The starting code is located in the `begin` folder. The `begin` code has been copied to the `end` folder for you. Please make your changes to the code in the `end` folder. The solution for the exercise is located in the `solution`  folder. The solution is provided as a reference to help you understand the requirements of the exercise. Try to complete the exercise without referring to the solution.

## Steps

1. Upgrade the application to assign new cars an id. The id should be a unique number starting at 1. The id should be assigned when the car is added to the vector of cars. Review the following expression to determine the max car `id` in the cars vector.

```rust
self.cars.iter().map(|car| car.id).max().unwrap_or(0);
```

2. Add a new command named `remove`. The command will prompt the user for the id of the car to remove. The command will remove the car from the vector of cars. Review the code below to help you implement the `remove` command. Some of the code has been removed and replaced with comments. Where appropriate, add the code to complete the implementation with the car tool application.

```rust
pub fn remove_car(&mut self) {
    // add code to get the id of the car to remove, suggested prompt: "Enter car id to remove: "
    let index = self.cars.iter()/* add code here to use position function to the get index of the car with the id */;
    match index {
        // car with id found
        Some(i) => {
            // add code to remove the car from the vector
            println!("Car with id {} removed successfully", id);
        }
        // no car with id found
        None => println!("Car with id {} not found", id),
    }
}
```

3. Add the ability to save the car data to a JSON file. Add new command named `save`. The command will prompt the user for the filename, and save the vector of cars in a JSON format.

4. Add the ability to load the car data from a JSON file. Add new command named `load`. The command will prompt the user for the filename, and load the vector of cars from a JSON file.

5. Run the program and test the functionality.