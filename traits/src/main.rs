// collections of methods representing a set of behaviors nessesary to complete a task
// data types can implement a trait to implement those methods
// generics use traits to specify the capabilities of unknown data types
// similiar to interfaces in C++ or Java
// use trait to abstractly define the behavior of data types to set bounds on data type when working with generics
// when you define a new custom data type with a struct, by default is does not implement any traits
// rust compiler is able ot provide default implementations for several common traits via derived attributes
// when you derive traits the compiler will provide default code for the required methods
// Derived traits: Eq, PartialEq, Ord, PartialPrd, Clone, Copy, Hash, Default, Debug
// Trait bounds require a generic type to implement specific traits
// Bounding guarentees the generic type will have necessary behaviors
//

struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32, // miles
}
#[derive(PartialEq, PartialOrd)] //will compare two instances of the struct and determine them to be equal only if all the fields are equal
struct Meteorite {
    name: String,
    mass: u32,
}

// single method named describe which returns a string
// trait is like a contract that says any data type that implements the description trait will have a method named describe that will return a string
trait Description {
    fn describe(&self) -> String {
        // end this line with ';' when there is no defualt implementation
        String::from("an object flying through space")
    }
}

// implement for Satellite data type using an implementation block
impl Description for Satellite {
    fn describe(&self) -> String {
        // within method use a format macro that says the satellite's name and it's velocity
        format!(
            "the {} is flying at {} miles per hour",
            self.name, self.velocity
        )
    }
}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "the {} is flying {} miles high with {} crew members aboard",
            self.name, self.altitude, self.crew_size
        )
    }
}

impl Description for Meteorite {}

fn main() {
    traits();
    trait_bounds();
    multiple_bounds();
    returning_traits();
}

// instantiates a Satellite to represent hubble and SpaceStation to represent iss
// print macro does not have ability to print a struct
// use a trait to get a formatted description of the struct

fn traits() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    let hoba = Meteorite {
        name: String::from("Hoba"),
        mass: 60000,
    };
    let capo_del_cielo = Meteorite {
        name: String::from("Campo del Cielo"),
        mass: 28840,
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
    println!("hoba is {}", hoba.describe());
    println!("hoba == capo_del_cielo is {}", hoba == capo_del_cielo);
    println!("hoba > capo_del_cielo is {}", hoba > capo_del_cielo);
}

// trait bounds
// accepts input item of generic type T and then prints a message with its data type using the any moduels type_name function which returns the name of a given type as a string slice
// input item is a parameter to the print macro so that item needs to be given as output- it has to implement display trait
// need to restrict type T to data types that implement display

use std::any;
use std::fmt; //display trait

fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}
fn trait_bounds() {
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    //print_type([13]); produces an error, can use debug trait instead
}

//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(
fn compare_and_print<T, U>(a: T, b: U)
where
    T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy,
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn multiple_bounds() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
}

// return types based on the traits it implements
// data types and trait bounds that cannot be known at runtime use yse dynamic dispatch- beyond the scope of this project
fn returning_traits() {
    println!("output is {}", get_displayable());
}

fn get_displayable() -> impl fmt::Display {
    13
}
