// binary of 0b and binary
// rust allows underscores in binary
// rust uses signed 32bit integer as default
// can specify smaller data type
// byte is 8 related bits
// can minupulate as individual bits ex. digital IO pins and data direction register

fn main() {
    let mut value = 0b1111_0101u8; // stored as an integer
    println!("Value is {}", value);
    println!("Value is {:08b}", value); // colon indicated special formatting, b indicates binary bits, 8 is bits to display, 0 is to display leading 0's

    // bitwise opperators to perform logical operations on patterns of mits at the individual bit level
    // NOT inverts value
    // represented by !
    value = !value;
    println!("Value is {:08b}", value);

    // AND takes two inputs A and B
    // if A and B = 1 then output is 1
    // if A or B = 1 then output is 0
    // if A and B = 0 then output is 0
    // can clear value of specific bit
    // represented by &

    value = value & 0b1111_0111;
    println!("Value is {:08b}", value);

    // can also check value of specific bit
    // used to check status registers containing information regarding current state of the processor
    println!("Bit 6 is {}", value & 0b0100_0000);

    // OR takes two inputs A and B
    // if either A or B = 1 then the output will be 1
    // if A and B = 0 then output is 0
    // can be used to change a specific bit to set it's value to 1
    // represented by |

    value = value | 0b0100_0000;
    println!("Value is {:08b}", value);

    // XOR exclusive OR takes two inputs A and B
    // if A and B = 1 or A and B = 0 then output will be 0
    // if only A or B = 1 then output will be 1
    // can check when value differs between two bit patters
    // represented by ^

    value = value ^ 0b0101_0101;
    println!("Value is {:08b}", value);

    // shift operators shift pattern left or right by a certain number of bits
    // backfilled shift filled by 0's
    // leftshift <<
    // rightshift >>

    value = value << 4;
    println!("Value is {:08b}", value);

    value = value >> 2;
    println!("Value is {:08b}", value);
}
