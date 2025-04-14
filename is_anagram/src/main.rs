//Write a function that takes two strings and returns true if
//they are anagrams of each other — that is, if both strings contain the same characters
//in any order (ignoring case and non-alphabetic characters).

use std::collections::HashMap;

fn main() {
    let string1: &str = "Listen";
    let string2: &str = "Silent";
    let result = is_anagram(&string1, &string2);
    println!("{}", result);
}

fn is_anagram(text1: &str, text2: &str) -> bool {
    let cleaned1: String = text1
        .to_lowercase() // Convert all characters to lowercase to make comparison case-insensitive
        .chars() // Convert the string into an iterator of characters
        .filter(|c| !c.is_whitespace()) // Keep only characters that are NOT whitespace (ignore spaces, tabs, newlines)
        .collect(); // Collect the filtered characters back into a new String

    let mut char_counts1: HashMap<char, usize> = HashMap::new();

    for c in cleaned1.chars() {
        *char_counts1.entry(c).or_insert(0) += 1;
    }

    let mut char_counts2: HashMap<char, usize> = HashMap::new();

    let cleaned2: String = text2
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    for c in cleaned2.chars() {
        *char_counts2.entry(c).or_insert(0) += 1;
    }

    char_counts1 == char_counts2
}
