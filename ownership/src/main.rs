// scope is the region of the program where a variable is valid
// variable becomes valid when it comes into scope
// variable remains valid until it goes out of scope
// variable bindings are constrained to live within a block of code
// blocks of code are encoded by {}
//
// variables are immutable until declared mutable
// shadowing is declaring a new variable with the same name as existing variable
// new variable masks the value of the first

fn main() {
    shadowing();
    stack();
    string();
    ownership();
    clone_ownership();
    ownership_issue();
    integer_stack_ownership();
    transfering_ownership_int();
    transfering_ownership_string();
}
fn shadowing() {
    let planet = "Earth";
    {
        println!("planet is {}", planet);
        let planet = "Mars";
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);
}

// program memory is stored in the stack and the heap
//
// stack:
// values stored in sequential order as they are recieved
// data added and removed as last in, first out (LIFO)
// like a stack of boxes full of boxes
// process of adding more boxes is called pushing data onto the stack
// after function2 is returned it is popped off the stack and function1 executes
// data can be pushed and popped quickly
// small sie
// data must have known, fixed size
// stores integer, floating point, boolean, char, array, tuple - fixed size known to compiler
fn stack() {
    let x = 13;
    function1();
}
fn function1() {
    let y = 3.14;
    let z = 'a';
    function2();
}
fn function2() {
    let arr = [1, 2, 3];
    println!("array first value is {}", arr[0]);
}

//heap
// like having boxes on shelves in a warehouse
// when memory space is found to store data, it is allocated to the heap
// accessing data is more complicated
// less oragnized as data can be anywhere
// pointers are a data type that stores a memory address, like an invetory sheet
// pointers can be stored on the stack
// accessing and adding is slower
// can dynamically add and remove data and resize data structures
// lots of memory space
// stores string
// string literal- hard coded into the executable, immutable, must be known before compilation
// string type- allocated on the heap, mutable, dynamically generated at runtime
// Char in string are stored in heap and indexed with the value
// pointer to message stored in stack
// points to address of index 0 so we can find it/ length of the string/ capacity allocated in the heap

fn string() {
    let mut message = String::from("Earth"); // :: is path operator that allows acces to from function
    println!("message is {}", message);
    message.push_str(" is home.");
    println!("message is {}", message);
}

// need to clean up memorgy management
// programmer is responsible for memory management ex. C++ malloc() and free() -- lots of controll but memory leaks/ invalid access
// garbage collection automatically cleans up memory ex. Java, Go -- easy but wasteful of memory
// rust uses ownership- variables are responsible for freeing their resources
// Rules:
// 1. Every value is 'owned' by one, and only one. variable at a time- called owner
// 2. When owner goes out of scope, the value is dropped
// Advantages: safe, efficient
// Disadvantages: requires understanding of ownership

fn ownership_issue() {
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
    }
    //println!("inner_planet is {}", inner_planet); inner planet no longer exists
}

// string type stores data on the heap
// Letters to spell Mercury exist on the heap
// Assigned to string variable on the stack named inner_planet which became it's owner
// A copy of this value is assigned while still on the stack from inner_planet to outer_planet
// Thus two variables on the stack point to the same index on the heap
// Ownership of inner_planet was transfered to outer_planet and inner_planet was removed
// Transfer of ownership is called a move

fn ownership() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        outer_planet = inner_planet;
        //println!("inner_planet is {}", inner_planet); will cause error after move
    }
    println!("outer_planet is {}", outer_planet);
}
fn clone_ownership() {
    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        outer_planet = inner_planet.clone(); // duplicates copy data on the heap, one for each variable
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}

// dont need to clone because data is stored in stack and does not reference heap
// value from inner_planet to outer_planet are not moved, but copied
// copying occures implicitly; cloning is explicit
fn integer_stack_ownership() {
    let outer_planet: i32;
    {
        let mut inner_planet = 1;
        outer_planet = inner_planet;
        inner_planet += 1;
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);
}

// stack: rocket fuel is an integer
// stack: value of rocket is passed to process fuel function as the copy variable propellant

fn transfering_ownership_int() {
    let rocket_fuel = 1;
    process_fuel_int(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}
fn process_fuel_int(mut propellant: i32) {
    propellant += 1;
    println!("processing propellant {} ...", propellant);
}

// without .clone()
// heap: rocket fuel is a string
// stack: tocket fuel points to index in heap
// stack: propellant points to heap data and takes ownership
// stack: propellant goes out of scope and pointer is dropped
// heap: data no longer has pointer and data is dropped

fn transfering_ownership_string() {
    let rocket_fuel = String::from("RP-1");
    process_fuel_string(rocket_fuel.clone());
    println!("rocket_fuel is {}", rocket_fuel); //without clone, print is trying to use rocket_fuel which is no longer a string
}
fn process_fuel_string(propellant: String) {
    println!("processing propellant {} ...", propellant);
}
