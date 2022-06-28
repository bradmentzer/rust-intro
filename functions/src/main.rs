// functions organize code into reusable modules
// must specify data type for all parameters

fn main() {
    let result = square(13);
    println!("result is {}", result);
}

// statements perform an action without performing a value, ends with a semicolon. Result cannot be used
// expressions evaluate with a resulting value, does not end with a semicolon

//fn statement_expression_example() {
//let sum = a + b; // the whole line is a statement containing the expression 'a+b'
//}

// use return values to get resulting output data back from a function
// need to specify data type for return result
// this function will return an i32
// when we end a funciton with an expression. the result will get passed out as the return value from that function

fn square(x: i32) -> i32 {
    println!("squaring {}", x);
    x * x
}
