# Exercise 1 - Capture Car Data from the User

## Instructions

To complete the lab exercise, review the in-class demonstrations. This exercise combines code from the various demonstrations. Also, you may used generative AI tools, searching, and official documentation to complete the exercise.

### Code Folders - PLEASE READ

For this exercise, you will be creating a new Rust project with the Cargo. Please name your folder as described in the exercise. This folder will need to be register with root folder's `Cargo.toml` file. This registration is need for the Rust Analyzer to work correctly in VS Code. The solution for the exercise is located in the `solution`  folder. The solution is provided as a reference to help you understand the requirements of the exercise. Try to complete the exercise without referring to the solution.

## Steps

1. Create a new Rust project in the `exercise_01` folder. Run the following command from the terminal window in the `exercise_01` folder:

    ```bash
    cargo new car_tool
    ```

2. Add the new Rust project, `exercises/exercise_01/car_tool` to the top of the workspace members of the root `Cargo.toml` file. Save the file. The Rust workspace should reload automatically.

    ```toml
    [workspace]

    members = [
        "exercises/exercise_01/car_tool",
        "demos_rust/01_hello_world",
    ```

3. In the `exercise_01/car_tool` folder, open the `main.rs` file. Above the main method in the source file there should be `Run` link. Click the `Run` link to run the program. The program should compile and run without any errors. The output should be `Hello, world!`.

    Note: If you do not see the `Run` link, double check the `Cargo.toml` file from the previous step and manually reload the Rust Analyzer workspace using the `rust-analyzer: Reload workspace` command from the VS Code Command Palette.

4. Define a new struct named `Car` with two fields:

    - `make`: String
    - `model`: String
    - `year`: u32
    - `color`: String
    - `price`: f64

5. When the `main` function executes, prompt the user to enter a car `make`, `model`, `year`, `color`, and `price`. With the user input, create a new `Car` struct instance using a `Car` struct associated function.

6. Write a method function to print the details of the car console.

7. Using  `loop` and `break`, allow the user to enter three cars. After each car is entered, print the car details to console. To implement the solution, you may need an `if` statement to check if the user has entered three cars. We did not cover `if` statements in the demos. Use your AI or search engine tooling to figure out how to use an `if` statement in Rust.

8. Run the program and test the functionality.