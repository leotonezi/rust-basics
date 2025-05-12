fn main() {
    let temp_f: f64 = 98.6;
    let temp_c: f64 = 37.0;

    let celsius = fahrenheit_to_celsius(temp_f);
    println!("{} Fahrenheit is {} Celsius", temp_f, celsius);

    let fahrenheit = celsius_to_fahrenheit(temp_c);
    println!("{} Celsius is {} Fahrenheit", temp_c, fahrenheit);
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}
