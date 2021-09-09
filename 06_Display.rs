use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 20);
    println!("Compare minmax:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
    println!();

    let bit_range = MinMax(-100, 100);
    let small_range = MinMax(-1, 1);
    println!("Compare minmax:");
    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = bit_range,
    );
    println!(
        "The big range is {big:?} and the small is {small:?}",
        small = small_range,
        big = bit_range,
    );
    println!();

    let point = Point2D { x: 1.1, y: 9.9 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}
