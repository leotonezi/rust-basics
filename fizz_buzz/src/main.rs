fn main() {
    let n: i32 = 5;
    fizz_buzz(n);
}

fn fizz_buzz(number: i32) -> () {
    let mut i: i32 = 1;

    while i <= number {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }

        i += 1;
    }
}
