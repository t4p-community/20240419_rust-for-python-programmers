// We need crates to perform the JSON serialization and deserialization.
// Crates are similar to Python packages, but not quite the same
// Crates and the cargo command line tool are used to manage dependencies in
//   Rust (similar to pip in Python)
// Crates are defined in the Cargo.toml file (similar to requirements.txt in Python)
// The cargo add command is used to add a dependency to the Cargo.toml file (similar to pip install)
// Crates are different from Python packages in that they are source code that is compiled into a binary

// The serde and serde_json crates are used to perform JSON serialization and deserialization
// cargo add serde --features derive
// cargo add serde_json

// serde is a framework for serializing and deserializing Rust data structures efficiently and generically.
// serde_json provides serialization and deserialization support for JSON.
// the traits Serialize and Deserialize are used to derive the serialization and deserialization code.
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};

// derive the Serialize and Deserialize traits for the Person struct
// the Debug trait is used to print the Person struct
// derive works by automatically implementing the trait for the struct
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let people = vec![
        // creating a "new" associated function is not necessary
        // Person::new("Alice", 25),
        // instead, we can use the struct literal syntax
        Person {
            name: "Alice".to_string(),
            age: 25,
        },
        Person {
            name: "Bob".to_string(),
            age: 30,
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
        },
    ];

    let people_serialized = serde_json::to_string(&people).unwrap();
    println!("{}", people_serialized);

    // write to people.json file
    write("people.json", &people_serialized).unwrap();

    // read from people.json file
    let people_serialized = read_to_string("people.json").unwrap();

    let people_deserialized: Vec<Person> = serde_json::from_str(&people_serialized).unwrap();
    for person in people_deserialized {
        // "{:?}" is used to print the Person struct, requires the Debug trait
        println!("{:?}", person);
    }
}
