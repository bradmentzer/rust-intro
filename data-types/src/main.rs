// characterized by number of bits
// unsigned -> only positive values
// signed -> positive and negative values

// 8-bit integers represent 2^8 = 256 possible values
// unsigned (u8) this is 0 -> 255
// signed (i8) this is -128 -> 127

fn main() {
    // let x: u8 = -10; // rust will prevent negative
    // println!("x is {}", x);
    overflow();
    compile();
    float();
    float_32();
    sig_digit();
    sig_fig();
    sig_fig_0();
    break_ex();
    arrangement();
}
fn overflow() {
    // let x: u8 = 1000; // rust will prevent overflow
    // println!("x is {}", x);
}
fn compile() {
    let mut x: u8 = 255;
    x = x + 1; // rust will compile then abort at line 3 from overflow
    println!("x is {}", x);
}

// Represent floats with IEEE 754 standard
// Rust uses f32 and f64
// Value stored as fractional and exponential components
// Fractional component has finite signicant value
// Precise up to 6-9 or 15-17 digits respectivley

fn float() {
    let x = 10.0; //stored as f64 by defualt but will print as int
    println!("x is {}", x)
}

fn float_32() {
    let x: f32 = 10.123; //stored as f32 by defualt but will print as int
    println!("x is {}", x);
}

fn sig_digit() {
    let a = 10;
    let b = 3;
    let c = a / b;
    println!("c is {:.3}", c); // will print to three significant digits
}

fn sig_fig() {
    let a = 10;
    let b = 3;
    let c = a / b;
    println!("c is {:8.3}", c); // will print 8 spaces before decimel
}

fn sig_fig_0() {
    let a = 10;
    let b = 3;
    let c = a / b;
    println!("c is {:08.3}", c); // will print 8 leading 0's before decimel
}

fn break_ex() {
    let a = 10;
    let b = 3;
    let c = a / b;
    println!("c is {:08.3}\na is {}", c, a);
}

fn arrangement() {
    let a = 10;
    let b = 3;
    let c = a / b;
    println!("c is {0:08.3}\na is {1},\nonce again, c is {0}", c, a); // defines positional arrangement
}
