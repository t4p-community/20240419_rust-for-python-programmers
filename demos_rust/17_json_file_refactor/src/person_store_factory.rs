use crate::person_json_file_store::PersonJsonFileStore;
use crate::person_store::PersonStore;
use crate::person_xml_file_store::PersonXmlFileStore;

// Box - smart pointer that points to heap-allocated data
// dyn - dynamic dispatch, a mechanism that selects which implementation of a polymorphic operation
//.      (method or function) to call at runtime
// Box<dyn PersonStore> - trait object
// Result - enum with two variants: Ok and Err
// Ok - contains a value
// Err - contains an error message
pub fn person_store_factory(file_path: &str) -> Result<Box<dyn PersonStore>, String> {
    match file_path {
        // match if - enables evaluating multiple conditions within the match
        path if path.ends_with(".xml") => Ok(Box::new(PersonXmlFileStore::new(file_path))),
        path if path.ends_with(".json") => Ok(Box::new(PersonJsonFileStore::new(file_path))),
        _ => Err("Invalid file extension".to_string()),
    }
}

/*

# Dynamic Dispatch in Rust

Dynamic dispatch can impact the execution speed of Rust programs.

Dynamic dispatch in Rust involves using trait objects (like `Box<dyn Trait>`), which requires a
level of indirection and can prevent certain compiler optimizations. This is because the exact
code to run isn't known until runtime, which means the compiler can't inline the function call,
and each method call requires looking up the function address in the vtable.

However, the impact on performance is usually small and won't be noticeable in most applications.
It's also worth noting that dynamic dispatch allows for more flexible and extensible code by
enabling polymorphism.

As always with performance considerations, it's best to write clear and correct code first, then
profile to find any actual performance bottlenecks. If dynamic dispatch proves to be a bottleneck
for your specific use case, then it might be worth exploring other design options.

*/
