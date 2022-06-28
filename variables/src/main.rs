// variables in rust are immutable by default
// declare mutable with keyword mut
// variables are case-sensitive and bust begin with a letter or underscore
// https://rust-lang.github.io/api-guidelines/naming.html
fn main() {
    let mut x = 10;
    println!("x is {}", x);
    x = 20;
    println!("x is {}", x);
}
