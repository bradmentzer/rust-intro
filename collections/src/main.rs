// vectors are collections of elements with the same data type
// stored in sequential order
// can dynamically change in size and does not have to be known at compile time
// stored in heap
/// https://doc.rust-lang.org/std/vec/struct.Vec.html

fn main() {
    vectors();
    hash_maps();
}
// push method will append to the back of the vector
fn vectors() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grisson"));
    astronauts.push(String::from("Glenn"));
    println!("astronauts is {:?}", astronauts);

    let last = astronauts.pop(); // pop last element of vector and assign ownership to last variable
    println!("third is {:?}", last);

    // let third = &astronauts[2]; will panic
    // get method will return an option enum with a reference to the value at the specified index
    let third = astronauts.get(2);
    println!("third is {:?}", third);

    // instead of instantiating a new vector and pushing data, can use vec macro
    let countdown = vec![5, 4, 3, 2, 1];
}

// HashMaps store datat in key -> value pairs
// provide key to lookup cooresponding vlaues
// one way functin
// look up without using index
// keys and values can be different data types
// all keys must be same data type
// all values must be same data type
// each key can only have one value associates with it at a time
// keys cannot be stored in relative order

use std::collections::HashMap;

// first argument is a key and second is value
fn hash_maps() {
    let mut missions_flown = HashMap::new();
    missions_flown.insert("Hadfield", 3);
    missions_flown.insert("Hurley", 3);
    missions_flown.insert("Barron", 0);
    println!("missions_flown is {:?}", missions_flown);

    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);

    // update hashmap by:
    //  1. overwriting existing key-value pain
    //  2. insert a new extry if a key does not exist
    //  3. modify a value based on its existing value
    // https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html

    // 1.
    missions_flown.insert("Barron", 1);
    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);

    // 2.
    missions_flown.entry("Stone").or_insert(2);
    println!("missions_flown is {:?}", missions_flown);

    // 3.
    let kayla = missions_flown.entry("Barron").or_insert(0);
    *kayla += 1; // dereference operator to access value in memory and increment by one
    let barron_missions = missions_flown.get("Barron");
    println!("barron_missions is {:?}", barron_missions);
}
