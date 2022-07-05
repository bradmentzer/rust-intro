use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // read file and build vector of individual words
    let contents = match env::args().nth(1) {
        // checks to make sure it was passed input argument for the file to open
        Some(f) => match fs::read_to_string(f) {
            // reads and converts all text to lowercase
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file : {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Program requires an argument: <file path>");
            std::process::exit(2);
        }
    };
    // takes lowercase string of file contents and uses split whitespace to separate words, which returns an iterator
    // collect method to turn iterator into vector of string slices
    // collect takes items from an iterator and puts it into a collection
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    // count how many times each unique word occures
    // initialize HashMap which will words stored as string slices for keys
    // value will be integer counting how many times the word has occured
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    // iterates through vector of words and increments by one if word already exists
    // new words get added with initial count one
    for word in all_words.iter() {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    // determine most commonly used words
    // initialize variable to keep track of top words and their count value
    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();
    // iterates through kv pair in hashmap
    // updates top count variable
    // clears and pushes the corresponting word onto top words vector
    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key);
        }
        // checks to see if word is same as the top count word
        else if val == top_count {
            top_words.push(key);
        }
    }

    println!("Top word(s) occurres {} times:", top_count);
    for word in top_words.iter() {
        println!("{}", word);
    }
}
