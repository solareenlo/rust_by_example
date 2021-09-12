fn main() {
    let names = vec!["Alice", "Bob", "Carole"];
    for name in names.iter() {
        match name {
            &"Bob" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    println!("");

    let names2 = vec!["Alice", "Bob", "Carole"];
    for name in names2.into_iter() {
        match name {
            "Carole" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names2);
    println!("");

    let mut names3 = vec!["Alice", "Bob", "Carole"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Alice" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names3);
}
