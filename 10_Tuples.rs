use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (i, b) = pair;
    return (b, i);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("long_tuple first value: {}", long_tuple.0);
    println!("long_tuple second value: {}", long_tuple.1);
    println!();

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    println!();

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));
    println!();

    println!("one element tuple: {:?}", (3u32,));
    println!("just an integer: {:?}", (3u32));
    println!();

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
    println!();

    let matrix = Matrix(1.1, 2.2, 3.3, 4.4);
    println!("{:?}", matrix);
    println!("{:}", matrix);
}
