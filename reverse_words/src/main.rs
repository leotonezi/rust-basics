fn main() {
    let input: &str = "hello world rustaceans";
    let output = reverse_words(&input);
    println!("{}", output)
}

fn reverse_words(text: &str) -> String {
    let mut string = String::from(text);
    let words: Vec<&str> = string.split(' ').rev().collect();
    let sentence = words.join(" ");
    return sentence;
}
