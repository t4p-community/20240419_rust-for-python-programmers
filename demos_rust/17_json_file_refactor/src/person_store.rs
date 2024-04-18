use crate::person::Person;

// PersonStore is a trait
// A trait is a collection of methods defined for an unknown type: Self
// When you implement a trait, you define the behavior of a type
// The trait PersonStore has two methods: write and read
// Traits are similar to interfaces in other languages
// Traits are similar to Python abstract base classes and protocols
pub trait PersonStore {
    // & get a reference without taking ownership
    // [] slice is view into a contiguous sequence of elements
    // Person is the type of object of the slice
    fn write(&self, people: &[Person]);
    fn read(&self) -> Vec<Person>;
}
