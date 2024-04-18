use crate::person::Person;
use crate::person_store::PersonStore;
use std::fs::{read_to_string, write};

pub struct PersonJsonFileStore {
    file_path: String,
}

impl PersonJsonFileStore {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

impl PersonStore for PersonJsonFileStore {
    fn write(&self, people: &[Person]) {
        let people_serialized = serde_json::to_string(&people).unwrap();
        write(&self.file_path, people_serialized).unwrap();
    }

    fn read(&self) -> Vec<Person> {
        let people_serialized = read_to_string(&self.file_path).unwrap();
        serde_json::from_str(&people_serialized).unwrap()
    }
}
