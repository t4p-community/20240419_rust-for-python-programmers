// cargo add xmltree

use crate::person::Person;
use crate::person_store::PersonStore;
use std::fs::{read_to_string, write};
use xmltree::{Element, XMLNode};

pub struct PersonXmlFileStore {
    file_path: String,
}

impl PersonXmlFileStore {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

fn new_with_text(tag: &str, text: &str) -> Element {
    let mut element = Element::new(tag);
    element.children.push(XMLNode::Text(text.to_string()));
    element
}

impl PersonStore for PersonXmlFileStore {
    fn write(&self, people: &[Person]) {
        let mut root = Element::new("people");
        for person in people {
            let mut person_element = Element::new("person");
            person_element
                .children
                .push(XMLNode::Element(new_with_text("name", &person.name)));
            person_element.children.push(XMLNode::Element(new_with_text(
                "age",
                &person.age.to_string(),
            )));
            root.children.push(XMLNode::Element(person_element));
        }

        let mut bytes = Vec::new();
        root.write(&mut bytes).unwrap();

        // Convert bytes to String
        let xml_string = String::from_utf8(bytes).expect("Found invalid UTF-8");
        write(&self.file_path, xml_string).unwrap();
    }

    fn read(&self) -> Vec<Person> {
        let xml_string = read_to_string(&self.file_path).unwrap();
        let root = Element::parse(xml_string.as_bytes()).unwrap();
        root.children
            // iter - creates an iterator
            .iter()
            // map - transforms each element of an iterator
            .map(|child| {
                // if-let - pattern that matches a value and executes code if it matches
                // XMLNode::Element(child_element) - pattern that matches an XMLNode::Element
                if let XMLNode::Element(child_element) = child {
                    let name = child_element
                        .get_child("name")
                        .unwrap()
                        .get_text()
                        .unwrap()
                        .clone();
                    let age = child_element
                        .get_child("age")
                        .unwrap()
                        .get_text()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap();
                    // return value, observe the missing semi-colon at the end of the expression
                    Person {
                        name: name.to_string(),
                        age,
                    } // no semi-colon
                } else {
                    // panic! - macro that prints an error message and exits the program
                    // the "child_element" did not match the pattern XMLNode::Element
                    panic!("Invalid XML format");
                }
            })
            // collect - collects the elements of an iterator into a collection
            .collect()
    }
}
