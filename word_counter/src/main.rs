// read the file path from command line , return the no of lines, words , and characters and also 5 most
// common words
use std::env;
use std::path::PathBuf;

use std::collections::HashMap;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("Please provide a file path");

    // Read file
    let contents = fs::read_to_string(path).expect("Could not read file");

    // Count lines, words and characters
    let lines = contents.lines().count();
    let words = contents.split_whitespace().count();
    let characters = contents.chars().count();

    // Count word frequencies
    let mut map: HashMap<String, usize> = HashMap::new();

    for word in contents.split_whitespace() {
        let word = word.to_lowercase();
        *map.entry(word).or_insert(0) += 1;
    }

    // Get 5 most common words
    let mut frequencies: Vec<_> = map.iter().collect();

    frequencies.sort_by(|a, b| b.1.cmp(a.1));

    println!("Lines: {}", lines);
    println!("Words: {}", words);
    println!("Characters: {}", characters);

    println!("\nTop 5 most common words:");

    for (word, count) in frequencies.iter().take(5) {
        println!("{}: {}", word, count);
    }
}
