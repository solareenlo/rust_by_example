fn main() {
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    println!("{}", *reference);
    println!("{}", reference);
    println!();

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    println!("{}", value);
    println!();

    let mut mut_value = 6;
    println!("{}", mut_value);
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
    println!("{}", mut_value);
}
