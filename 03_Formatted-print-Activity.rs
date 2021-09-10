#[derive(Debug)]
struct Structure(i32);

fn main() {
    println!("This struct `{:?}` won't print...", Structure(3));

    let pi = 3.141592;
    println!("{:.3}", pi)
}
