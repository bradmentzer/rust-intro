fn main() {
    if_true();
    if_not_true();
    if_multiple_conditions();
}
fn if_true() {
    let x = 4;
    if x == 3 {
        println!("x is 3"); // print if x is 3
    }
}
fn if_not_true() {
    let x = 4;
    if x + 1 != 3 {
        println!("x + 1 is 3"); // print if x is not 3
    }
}
fn if_multiple_conditions() {
    let x = 3;
    let y = 5;
    if x > y {
        println!("x is greater than y");
    } else if y > x {
        println!("y is greater than x");
    } else {
        println!("x is equal to y");
    }
}
fn if_shorthand() {
    let make_x_odd = true;
    let x = if make_x_odd { 1 } else { 2 };
    println!("x is {}", x);
}
