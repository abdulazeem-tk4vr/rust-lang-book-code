pub mod structs;
pub mod enums;
pub mod lifetimes;

use lifetimes::start as life; // Alias the 'start' function from the 'lifetime' module as 'life'
use enums::start as enum_func;
use structs::start as struct_func;

fn main() {
    println!("Hello, world!");
    struct_func();
    enum_func();
    life(); // Use the 'life' alias to call the 'start' function from the 'lifetime' module
}


/*

structs - involves basic playing around with structs, using default values and assigning values for the rest of the struct
enums - enum, pattern matching, result, option, format! and custom enums for returning err.

lifetime - traits, trait bounds, generic types and lifetimes
collections - vector, hashmap, string ( slices and objects)
closures - closures, iterators, 
smart_pointers - 

*/