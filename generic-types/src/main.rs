// flexability to define structs with either integers or floats
// abstract stand-ins fro concrete data types or props
// defined with <T>
// generics are a zero-cost abstraction
// compiler replaces generic placeholders with concrete data types

// generics can be used to represent fields within a struct
#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U,
}

// generics can be used to represent methods within a struct
// return type will data type T and will return a reference to the width value
// it has to be a reference becuase we dont yet know what data type T is
impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

//methods defined in this block will only be avaliable to recangles with width and height u8
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    reactangle_fn();
    reactangle_u8();
    print_biggest();
    box_type();
}

fn reactangle_fn() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
}

fn reactangle_u8() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u8,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
}

//generic to define function
//binary comparison operator cannot be use with type T
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
fn print_biggest() {
    println!("biggest is {}", get_biggest(1, 2));
}

// Box<T> data type allows to store some data type <T> on the heap instad of the stack
// Box coinains pointer on the stack which points to a chunck of memory on the heap which is large enough to hold memory of type T
// Boxes are 'smart pointer'
// Box<T> has ownership of the data it points to
// When Box<T> goes out of scope it deallocates the heap memory

// Box<T> can be used to store a data type whose size cannot be known at compile time
// Example: Recursive type- like a struct that contains a struct as one of its fields
// Can transfer ownership of data rather than copying it on the stack
// Avoid copying large amounts of stack data

use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn box_type() {
    let vehicle = Shuttle {
        name: String::from("Atlantas"),
        crew_size: 7,
        propellant: 0.0,
    };
    println!(
        "vehicle is size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );
    //program is allocating nessesary amount of memory on the heap for a shuttle and places existing instance into it. Vehicle variable loses ownership of the struct
    // boxed_vehicle becomes new owner which uses the box pointer which lives on the stack
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    println!(
        "boxed_vehicle is size on stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );
    // dereference operator represented with *
    // when applied to a pointer it denotes the pointed-to location
    println!(
        "boxed_vehicle is size on heap: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );

    // move box data back from the heap to the stack by dereferencing it
    // pass ownership to unboxed vehicle on the stack
    let unboxed_vehicle: Shuttle = *boxed_vehicle;

    println!(
        "unboxed_vehicle is size on stack: {} bytes",
        mem::size_of_val(&unboxed_vehicle)
    );
}
