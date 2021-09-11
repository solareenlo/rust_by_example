#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 30;
    let peter = Person { name, age };
    println!("{:?}", peter);
    println!("{:#?}", peter);
}
