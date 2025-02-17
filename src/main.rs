mod collection;

use collection::collection_demo;
use quick_lib::{indirect_access, public_function, person::Person};
use quick_lib::person::Gender;

fn main() {
    println!("Hello, World!");
    collection_demo();
    println!("This is a example for quick_lib crate");
    public_function();
    indirect_access();
    let programmer: Person = Person::new("Vikas".to_owned(), 20, Gender::Male);
    println!("Name: {}", programmer.get_name());
    println!("Age: {}", programmer.get_age());
}