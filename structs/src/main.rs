//tuples are used to group multiple items of mixed data types
// elements are ordered
// structs group multiple sata types
// elements are named and order is not relevent
// structs are made up of fields which contain a name ans associated data type

// Struct data is stored on the stack
// If stuct contains heap-based data, then the pointer will live on the stack with the rest of the struct
// The content of the string will be on the heap
// when stuct goes out of scope, heap data is dropped

#[derive(Debug, Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    struct_example();
    methods();
    associated_functions();
    tuple_struct();
}

fn struct_example() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 832490.0,
    };
    let vehicle2 = Shuttle {
        ..vehicle.clone() // any values that are not set should have the same value as the first vehicle. need to clone for same name because strings are on the heap
    };

    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
}

// methods are subroutine associated with a struct
// can have inport parameters and return a value
// declaered with fn
// first parameter is a ref to a struct

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0,
        }
    }
}

fn methods() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 0.0,
    };
    let vehicle_name = vehicle.get_name();
    println!("vehicle name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);
}

// associated functions are functions associated with a struct data type
// does not have a &self parameter
// cannot use to ref data within instance of a struct
// provide subroutiens that are related to the struct data type
// can be used to define constuctor to build new instance of a struct

fn associated_functions() {
    let mut vehicle = Shuttle::new("Associated Function");
    let vehicle_name = vehicle.get_name();
    println!("vehicle name is {}", vehicle_name);
}

// tuple structs store a collection of mixed data without named fields
// distiguishable as a uniqure data type

struct Color(u8, u8, u8); // RGB
struct Point(u8, u8, u8); //xyz coordinate

fn get_y(p: Point) -> u8 {
    p.1
}

fn tuple_struct() {
    let red = Color(255, 0, 0);
    println!("Red value is {}", red.0);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {}", y)
}
