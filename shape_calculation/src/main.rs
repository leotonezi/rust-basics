use std::f64::consts::PI;

trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64
}

struct Rectangle {
    width: f64,
    height: f64
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * (self.radius * self.radius)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area<T: Area>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn main() {
    // Create instances of Circle and Rectangle
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    // Call the print_area function
    print_area(&circle);
    print_area(&rectangle);
}
