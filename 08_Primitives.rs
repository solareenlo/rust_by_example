use std::any::type_name;

fn print_out<T: std::fmt::Debug>(s: &str, arg: T) {
    println!("{0} is {1:?}, it's type is {2}", s, arg, type_name::<T>());
}

fn main() {
    let logical:bool = true;
    print_out("logical", logical);

    let a_float: f64 = 1.0;
    print_out("a_float", a_float);

    let an_integer = 5i32;
    print_out("an_integer", an_integer);

    let default_float = 2.0;
    print_out("default_float", default_float);

    let default_integer = 4;
    print_out("default_integer", default_integer);

    let mut inferred_type = 10;
    print_out("inferred_type", inferred_type);
    inferred_type = 423i64;
    print_out("inferred_type", inferred_type);

    let mut mutable = 11;
    print_out("mutable", mutable);
    mutable = 12;
    print_out("mutable", mutable);

    let mutable = true;
    print_out("mutable", mutable);
}
