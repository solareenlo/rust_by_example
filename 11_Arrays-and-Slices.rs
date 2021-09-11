use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("number of elements in array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);
    println!("{}", xs[4]);
    println!();

    let ys: [i32; 500] = [0; 500];
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
    println!("{}", ys[100]);
}