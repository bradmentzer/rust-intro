//
fn main() {
    if_loop();
    while_loop();
    while_array();
    for_loop();
    for_loop_index_enumerated();
    nested_loop();
}

// loop: Repeat a block of code forever
// Need the loop to return a value
fn if_loop() {
    let mut count = 0;
    loop {
        if count >= 10 {
            break;
        }
        count += 1;
        println!("count is {}", count);
    }
    println!("After the loop");
}

// while: continue repeating a block of code as long as a condition is true

fn while_loop() {
    let mut count = 5;

    while count < 10 {
        count += 1;
        println!("count is {}", count);
    }
}
fn while_array() {
    // may lead to bugs when indexing array elements
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}

// for: iterate over each item in a collection
// repeat a block of code N times
// converts message array into iterator
// implements logic to iterate over each item in a collection
// next() method returns the next item in the sequence
// https://doc.rust-lang.org/std/iter/trait.IntoIterator.html

fn for_loop() {
    let message = ['h', 'e', 'l', 'l', 'o'];
    for item in message {
        println!("item is {}", item);
    }
    for number in 0..5 {
        println!("number is {}", number); // range
    }
}
fn for_loop_index_enumerated() {
    let message = ['h', 'e', 'l', 'l', 'o'];
    for (index, &item) in message.iter().enumerate() {
        //& needed to break at char whie enumerating
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }
}

// nested loops to process elements of multi-dimentional array
fn nested_loop() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix {
        for mut num in row {
            num += 10;
            print!("{}\t", num) //\t will insert tab character after number
        }
        println!();
    }
}
