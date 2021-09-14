use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    let array = [1, 2, 3];
    let string = "hello";

    vec.print_in_option();
    array.print_in_option();
    string.print_in_option();
}
