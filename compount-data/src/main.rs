// arrays are a collection of elements with the same data type
// elements are stored in order
// stored in contiguous memory locations
// fixed length and cannot be dynamically resized
// can hold time series data

fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x'; // first letter x before printing
    let first_letter = letters[0];
    println!("first_letter is {}", first_letter); //firsts_letter is x

    let numbers: [i32; 5]; //array of length 5
    numbers = [0; 5]; // fills array with 0
    println!("last number is {}", numbers[4]);

    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    let lot_num = parking_lot[0][1];
    println!("lot num is {}", lot_num);

    let _garage: [[[i32; 100]; 20]; 5]; // contains 3D array with 5 x 20 x 100 integer elements

    let _parking_garage = [[[0; 100]; 20]; 5]; // 5 x 20 x 100 array with value 0 for each spot

    // Tuples group multiple items of mixed data types
    // Elements have relative order stored in a fixed-length, contiguius section of memory
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x');
    stuff.0 += 3; //retrieve value from valiable to the left, add three, and return value
    let first_item = stuff.0;
    println!("first_item is {}", first_item);

    //common for destructuring

    let (a, b, c) = stuff;
    println!("b is {}", b);
}
