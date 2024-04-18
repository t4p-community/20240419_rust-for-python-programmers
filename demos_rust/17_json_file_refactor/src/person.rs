use serde::{Deserialize, Serialize};

// derive - automatically implement traits for a type
// Serialize - trait that allows a type to be converted into a JSON string
// Deserialize - trait that allows a JSON string to be converted into a type
// Debug - trait that allows a type to be printed for debugging
#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: i32,
}
