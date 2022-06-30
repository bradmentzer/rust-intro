// returns an iterator over arguments passes to the program
// first argument is traditionally the executable path
// nth method returns nth item from an iterator

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    //args_fn();
    file_reading();
}

fn args_fn() {
    if env::args().len() <= 2 {
        println!("Program requires at least 2 arguments");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        // call args returns iterator, call enumerate to get indicies and use for loop to iterate thorugh and print each argument
        println!("argument {} is {}", index, argument)
    }
    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}

// reading from files https://doc.rust-lang.org/std/fs/
// https://doc.rust-lang.org/std/path/

fn file_reading() {
    let contents = fs::read_to_string("planets.txt").unwrap();
    println!("contents is {}", contents);

    for line in contents.lines() {
        println!("line is {}", line);
    }

    let contents = fs::read("planets.txt").unwrap(); // read returns a vector of u8 values representing the bytes in the file
    println!("contents is {:?}", contents); // print debug formatter because it is unable to display vectors

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open("planets.txt")
        .unwrap();
    file.write(b"\nPluto");
}
