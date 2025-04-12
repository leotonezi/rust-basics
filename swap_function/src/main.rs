fn main() {
    let mut a = 10;
    let mut b = 20;

    println!("Before swap: a = {}, b = {}", a, b);
    swap(&mut a, &mut b);
    println!("After swap: a = {}, b = {}", a, b); // a = 20, b = 10
}

fn swap(a: &mut i32, b: &mut i32) {
    let aux: i32 = *b;
    *b = *a;
    *a = aux;
}
