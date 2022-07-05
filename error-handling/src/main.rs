// rust does not use traditional prgramming exceptions to handle errors
// two types of runtime errors after compilation: recoverable and unrecoverable
// recoverable: problems that program can do something to resolve
//      handle with result enum type Result<T,E>
//      ex. file not found error, try a different path
// unrecoverable: errors that arise because rust does not allow the action
//      handle with panic macro panic!
//      ex. index beyond array bounds
use std::fs;
use std::io;

fn main() {
    panic_macro();
    match_error();
    kind_method();
    propagating_errors();
}

// panic! immediatley terminates the progeam and provides feedback on what went wrong
// backtrace is a log of all the functions that have been called up till error calling panic
// `env RUST_BACKTRACE=1`
fn panic_macro() {
    //panic!("Houston, we've had a problem!");
    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println! {"T-minus {}", count};
        //let x = 1 / count; will panic
    }
}

// Result enum has two varients: Ok and Err which represents restult that can sucees or fail
// Ok stores with generic type (T) and error varint stores error value generic type (E)
// .unwrap method examines if the result of an enum is OK and returns its stored value, if the result is an error, it will panic
// .expect is similar to .unwrap and takes an input for a custom error mesaage to display
// best practice is to use match expression to handle error result
// https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html
fn match_error() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    let contents = match result {
        Ok(mesaage) => mesaage,
        Err(error) => String::from("Nobody knows the ultimate question"),
    };

    println!("contents is {:?}", contents);
}

fn kind_method() {
    let result = fs::read_to_string("the_ultimate_question.txt");

    let contents = match result {
        Ok(mesaage) => mesaage,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found"),
            io::ErrorKind::PermissionDenied => String::from("Permission denied"),
            _ => panic!("Another type of errir {:?}", error),
        },
    };

    println!("contents is {:?}", contents);
}

fn propagating_errors() {
    let result = read_and_combine("planets.txt", "dwarf_planets.txt");
    match result {
        Ok(s) => println!("result is... \n{}", s),
        Err(e) => println!("There was ann error: {}", e),
    };
}

fn read_and_combine(f1: &str, f2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(f1)?; // match shorthand
    let mut s2 = match fs::read_to_string(f2) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    s1.push('\n');
    s1.push_str(&s2);
    Ok(s1)
}
