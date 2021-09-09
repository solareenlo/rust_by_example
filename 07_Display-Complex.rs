use std::fmt;

#[derive(Debug)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

fn main() {
    let com = Complex { re: 10.0, im: 10.0 };
    println!("Display: {}", com);
    println!("Debug: {:?}", com);
}
