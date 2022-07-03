// dangling reference is when you try to point to a reference after it has already gone out of scope
// borrow checker is used by compiler to compare scope to determine whether all borrows are valid
// Rust considers scope of variables and the lifetime of the values they refernece using borrow checker

fn main() {
    borrow_checker();
    lifetime_annotation();
    mult_lifetime_annotation();
    struct_lifetime();
}

fn borrow_checker() {
    let propellant; // declared but not initialized
    {
        let rp1 = String::from("RP-1"); // propellant varialble borrows
        propellant = &rp1;
        println!("propellant is {}", propellant);
    } //println!("propellant is {}", propellant); rp1 does not live long enough
}

// Compiler will sometimes mandate annotating lifetime of variables
// Return a borrowed reference where its not possible to know which borrowed value will be x or y
// define the lifetime in angle brackets after the function name and associate it with parameters
// Lifetime annotation: explicitly defines a generic lifetime for parameters
// Must begin with an apostrophe (') symbol
// Names are conventionally single lowercase letters

fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a defines a generic lifetime for input and output for input and output parameters
    // placed immediatly after the borrow operator and contans a spect separating it from the data type
    // telling compiler how lifetime of input parameters relate to each other
    // lifetime of returned reference will be as long as the lifetime of the two input parameters x and y
    // will use most restrictive, or shorter, lifetime
    // borrow checker will assure wherever the reference is used, the value it points to will still be alive
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_annotation() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LGN");
    result = best_fuel(&propellant1, &propellant2);
    println!("result is {}", result);
}

// multiple lifetime annotations
// removes possibility that best_fuel could return the second input parameter
// x, y, and return reference, all share the same generic lifetime, ('a), in last example
// lifetime of returned reference is not dependent on the lifetime of y

fn mult_best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x // removes possibility that best_fuel could return the second input parameter
    }
}

fn mult_lifetime_annotation() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LGN");
    result = mult_best_fuel(&propellant1, &propellant2);
    println!("result is {}", result);
}

// borrow checker can sometimes infer the lifetime and annotating lifetime isnt necessary

// Lifetime Elison Rules: set of rules for the compuler to analyze reference lifetimes- do not requre lifetime annotations
// 1. Each input parameter that is a reference is assigned its own lifetime
//     fn get_first_word<a'>(x: &'a str) -> &str {
//          a function with onl one reference parameter only needs one lifetime
//     fn get_first_word<'a>(x:&'a str, y i32) -> &str{
//          only applies to lifetimes that are references
//     fn get_longest<'a, 'b>(x: &'a str, y:&'b str) -> &str{
//          functions with two reference parameters get two distinct lifetimes

// 2. If there is exactly one input lifetine, assign it to all output lifetimes
//     fn get_first_word<a'>(x: &'a str) -> &str {
//          the lifetime of an output refernece needs to be as long as the input reference
//          compiler is able to infer all of the lifetime references so lifetime can be omited
//     fn get_longest<'a, 'b>(x: &'a str, y:&'b str) -> &str{
//          does not meet elision requirement because its assigned two different lifetimes

// 3. If one of the input parameters is a reference to self (&self) or mutable self (&mut self) its lifetime will be assigned to all output lifetimes
//     fn send_transmission(&self, msg: &str) -> {
//          must annotate
//          assigned two input lifetimes by rule #1
//          lifetime assigned to self 'a with a return value
//          this is intended to cover the common use case where a method returns the value from one of a struct fields, in this case is a reference
//          if the reference that the method is returning is not from the struct itself, it will require annotations

// When struct stores data using string data type, it has ownership of that string
// When struct goes out of scope, string data is removed from the heap

struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&self, msg: &'b str) -> &'b str {
        println!("Trandmitting message: {}", msg);
        msg
    }
}

fn struct_lifetime() {
    let vehicle = Shuttle { name: "Endeavour" };
    let sender = vehicle.send_transmission("Greetings from orbit");
    println!("sender is {}", sender);
}

// 'static lifetime indicates references avaliable for entire duration of the program
// example: string literal
// let s: &'static str = "this is static"
// ensures the data type will olny contain 'static elements
// can be used as a trait bout when defining generic types to ensure that the reciever will have value throughout project
// T: Display + 'static
