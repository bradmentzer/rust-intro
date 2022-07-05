// an enumeration defines a data type with multiple possible variants
// commonly used in match expressions which can control the flow of a program similar to a sequence of if else expressions
// match operator compares a value to a series of patterns to determine which code to exicute- similar to switch statment

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn get_perimeter(&self) -> f64 {
        match self {
            //comiler will automatically refernece and dereference match patterns
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(a, b, c) => a + b + c,
        }
    }
}

fn main() {
    enum_shape();
    enum_number();
    option_enum();
    matching_options();
    match_enum();
    if_let_expression();
}

fn enum_shape() {
    let my_shape = Shape::Rectangle(1.2, 3.4);
    println!("my_ shape is {:?}", my_shape);

    match my_shape {
        Shape::Circle(r) => println!("This is a circle with radius {}", r),
        Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
        Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c),
    }

    let perimeter = my_shape.get_perimeter();
    println!("perimeter is {}", perimeter);
}

fn enum_number() {
    let my_number = 1u8;

    let result = match my_number {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => {
            // wildcard
            println!("{} did not match", my_number);
            "something else"
        }
    };
    println!("result is {}", result);
}

// rust does not have a traditional nulll value
// rust uses Option<T> Enum to represent null and not null and comiler can check if it is correct
// option enum used with 'Some(value_to_store)' or 'let nothing = None'
// get method can be used on slices to return an option enum holdging a refernce to the value at the specified index
// cannot add integer to option enum holding integer
// option enum is its own data type

fn option_enum() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5); // index beyond length of the array, so theres nothing to get
    let number = number.unwrap_or(&0) + 1; // if None, then 0 is used
    println!("number is  {:?}", number);
}

fn matching_options() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = number.unwrap_or(&0) + 1;
    println!("number is  {:?}", number);
}

// evaluate is variable is Some or None
fn match_enum() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = match number {
        Some(number) => number + 1,
        None => 0,
    };
    println!("number is {:?}", number);
}

// instantiate option enum with integer value 13 and use match expression to evaluate it
fn match_expression() {
    let number = Some(13);

    match number {
        Some(13) => println!("thirteen"),
        _ => (),
    }
}

// write expressions checking for a single condition
// https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
fn if_let_expression() {
    let number = Some(13);

    if let Some(13) = number {
        println!("thirteen");
    }
}
