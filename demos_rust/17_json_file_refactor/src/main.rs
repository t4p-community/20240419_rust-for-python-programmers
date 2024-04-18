mod person;
mod person_json_file_store;
mod person_store;
mod person_store_factory;
mod person_xml_file_store;

use person::Person;
use person_store_factory::person_store_factory;

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

    let person_store = person_store_factory("people.json").expect("Unsupported file type.");
    person_store.write(&people);

    let people_deserialized = person_store.read();
    for person in people_deserialized {
        println!("{:?}", person);
    }

    let person_store = person_store_factory("people.xml").expect("Unsupported file type.");
    person_store.write(&people);

    let people_deserialized = person_store.read();
    for person in people_deserialized {
        println!("{:?}", person);
    }
}
