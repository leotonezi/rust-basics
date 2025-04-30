fn main() {
    let numbers = vec![3, 7, 2, 9, 4];
    match min_max(&numbers) {
        Some((min, max)) => println!("Min: {}, Max: {}", min, max),
        None => println!("The list is empty!"),
    }
}

fn min_max(numbers: &[i32]) -> Option<(i32, i32)> {
    let min = numbers.iter().min()?;
    let max = numbers.iter().max()?;
    Some((*min, *max))
}
