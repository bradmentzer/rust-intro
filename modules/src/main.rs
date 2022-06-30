// rust standard library contains core data types, fucntions, macros, etc
// https://doc.rust-lang.org/std/
// bring modle path into scope using 'use'
// std is avaliable to Rust by default
// prelude is list of common modules automatically imported

// standard input module
// if we need to use input as a different data type, need to parse
use rand::prelude::*;
use std::io;

fn main() {
    buffer();
    rand_crate();
}

fn buffer() {
    let mut buffer = String::new();
    println!("Enter a message!");
    io::stdin().read_line(&mut buffer); // when functions reads line of input, will update buffer with that string
    println!("Buffer is {}", buffer);

    let number: i32 = buffer.trim().parse().unwrap(); // trim to get rid of new line character before parsing,
    println!("number + 1 is {}", number + 1);
}

//Creates are a collection of Rust source fode files
// Binary crates compule to produce an executable program
// Library creates contain code for other programs
// https://crates.io/ is the official site for crates

fn rand_crate() {
    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11); // rand int 1-10
    println!("number is {}", number);
}
