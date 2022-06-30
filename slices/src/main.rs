// a slice is a reference to a contiguous section of a collection
// commomly encoutered as the string slide data type: &str
// string literals are slices
// data for string literal is hard coded into executable binary and string slice acceses it
// a borrowed reference to a string (&String) is not the same as a string slice (&str)
// a borrowed string reference points to an actual string on the stack, which points to and owns string data on the heap. String also stores length and capacity of heap data.
// slice stores pointer to heap data and length data. Does not track capacity because it will never own anything on the heap
// can use &String in places asking for &str (deref coersion)
// cannot use &str for &String
// when writing functions to work with strings without taking ownership, use slices for input and output
fn main() {
    slice();
    slice_as_param();
}

// message variable is on the stack pointing to the heap
// slice last_word points to the 15 index and contains length of the slice
fn slice() {
    let message = String::from("Greetings from Earth");
    println!("message is {}", message);

    let last_word = &message[15..]; // range is to the end of the message can also do [15..15+5]to specify
    println!("last word is {}", last_word);
}

fn slice_as_param() {
    let message = String::from("Greetings from Earth");
    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}
// takes a borrowed reference of an entire string as reference and returns first word without taking ownership of the string
// converts string to slice of bytes and iterates through contents checking for byte character representing space
// returns slice of input string ranging from begining to the index of the first space
// if no space is found, input was one word, and whole string is returned
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; //found a space
        }
    }
    &s //no spaces found; input is a single word
}

fn trim() {}
