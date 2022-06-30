// borrow is access and use data without ownership
// borrow operator: &
// when creating a mu ref, you cannot create other refs
// prevents data races
// can use: let ref1 = &mut var
// can use: let ref1 = &var; let ref2 = &var
// cannot use: let ref1 = &mut var; let ref2 = &var

fn main() {
    borrowing_references();
    return_ref();
}

// we dont need to need to transfer ownership of data to use it in a function
// change the value of a borrowed variable by marking it as mut
fn borrowing_references() {
    let mut rocket_fuel = String::from("RP-1");
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {} ...", propellant);
    propellant.push_str("is highly flamable");
    let length = propellant.len();
    length
}

// compiler assures data does not go out of scope
// cannot use borrow because data would go out of scope
fn return_ref() {
    let rocket_fuel = return_process_fuel();
    println!("rocket_fuel is {}", rocket_fuel);
}

fn return_process_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}
