fn main() {
    let n: i32 = 3;
    let result = is_even(&n);
    println!("{}", result);
}

fn is_even(number: &i32) -> bool {
    number % 2 == 0
}
