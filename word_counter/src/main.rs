fn main() {
    let string = "Boa noite";
    let counter = word_counter(&string);
    println!("{}", counter);
}

fn word_counter(input: &str) -> usize {
    input.split_whitespace().count()
}
